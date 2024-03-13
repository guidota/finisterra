use std::collections::VecDeque;

use engine::{
    camera::{self, Viewport, Zoom},
    draw::{image::DrawImage, Position, Target},
    engine::GameEngine,
    input::keyboard::KeyCode,
    CursorIcon,
};
use lorenzo::character::{animation::CharacterAnimation, direction::Direction};
use nohash_hasher::IntMap;

use crate::{
    game::Context,
    ui::{
        bar::Bar,
        button::Button,
        colors::*,
        console::Console,
        fonts::{TAHOMA_BOLD_8_SHADOW_ID, TAHOMA_REGULAR_10_ID, WIZARD_16_ID},
        input_field::InputField,
        inventory::Inventory,
        label::Label,
        spellbook::Spellbook,
        textures::{
            DV_BACKGROUND_ID, EXP_BAR_ID, INPUT_ID, INVENTORY_BUTTON_ID, MAIN_UI_ID,
            SPELLS_BUTTON_ID,
        },
        Alignment, Widget, UI,
    },
};

use self::entity::{Character, Entity};

use super::GameScreen;

pub mod entity;

pub struct WorldScreen {
    ui: WorldUI,

    entities: IntMap<u32, Entity>,

    move_keys: VecDeque<Direction>,

    me: u32,
}

pub struct WorldUI {
    x: u16,
    y: u16,

    // header
    level: Label,
    name: Label,
    desc: Label,

    // inventory
    inventory_button: Button,
    inventory: Inventory,

    // spellbook
    spells_button: Button,
    spellbook: Spellbook,

    // stats
    exp_bar: Bar,
    energy_bar: Bar,
    mana_bar: Bar,
    health_bar: Bar,
    agility: Label,
    strength: Label,
    gold: Label,

    // info
    fps: Label,
    ping: Label,

    console: Console,
    message_input: Option<InputField>,
}

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 540;

const TILE_SIZE: u16 = 32;
const WORLD_RENDER_WIDTH: u16 = 17 * TILE_SIZE; // 17 tiles
const WORLD_RENDER_HEIGHT: u16 = 16 * TILE_SIZE; // 16 tiles

impl WorldScreen {
    pub fn new<E: GameEngine>(engine: &mut E, entity_id: u32, mut character: Character) -> Self {
        let ui = WorldUI::initialize(engine, &character);
        let mut entities = IntMap::default();
        character.add_dialog(
            engine,
            "Rahma Nañarak O'al",
            TAHOMA_BOLD_8_SHADOW_ID,
            BLUE_3,
        );
        entities.insert(entity_id, Entity::Character(character));

        engine.set_mouse_cursor(CursorIcon::Default);

        Self {
            ui,
            entities,
            me: entity_id,
            move_keys: VecDeque::new(),
        }
    }

    fn prepare_viewports<E: GameEngine>(&mut self, engine: &mut E) {
        let size = engine.get_window_size();
        let zoom = if size.height >= (SCREEN_HEIGHT * 2) && size.width >= (SCREEN_WIDTH * 2) {
            engine.set_camera_zoom(Zoom::Double);
            2
        } else {
            engine.set_camera_zoom(Zoom::None);
            1
        };

        let x_start = std::cmp::max(0, (size.width / zoom - SCREEN_WIDTH) / 2);
        let y_start = std::cmp::max(0, (size.height / zoom - SCREEN_HEIGHT) / 2);
        self.ui.x = x_start;
        self.ui.y = y_start;

        engine.set_ui_camera_viewport(Viewport {
            x: 0.,
            y: 0.,
            width: size.width as f32,
            height: size.height as f32,
        });

        let world_camera_viewport = Viewport {
            x: (x_start as f32 + 14.) * zoom as f32,
            y: (y_start as f32 + 14.) * zoom as f32,
            width: WORLD_RENDER_WIDTH as f32 * zoom as f32,
            height: WORLD_RENDER_HEIGHT as f32 * zoom as f32,
        };
        engine.set_world_camera_viewport(world_camera_viewport);
    }

    fn update_character<E: GameEngine>(&mut self, context: &mut Context<'_, E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.update(context.engine);
            // camera follows main character
            context.engine.set_world_camera_position(camera::Position {
                x: character.render_position.0.floor() as f32,
                y: character.render_position.1.floor() as f32,
            });

            self.ui.update_character(context, character);
        }
    }

    fn update_fps<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let fps = format!("{:.0}", 1. / context.engine.get_delta().as_secs_f64());
        self.ui.fps.set_text(&fps, context.engine);
    }

    fn update_ping<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let ping = context.connection.ping();
        self.ui.ping.set_text(&format!("{ping}"), context.engine);
    }

    fn update_message_input<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let message_input_open = self.ui.message_input.is_some();
        let enter_pressed = context.engine.key_pressed(KeyCode::Enter);
        match (message_input_open, enter_pressed) {
            (true, true) => {
                if let Some(input) = self.ui.message_input.as_mut() {
                    let message = input.text();
                    if !message.is_empty() {
                        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me)
                        {
                            character.add_dialog(
                                context.engine,
                                message,
                                TAHOMA_BOLD_8_SHADOW_ID,
                                GRAY_6,
                            );
                        }
                    }
                }
                self.ui.message_input = None;
            }
            (false, true) => {
                let mut input_field = InputField::new(
                    GRAY_6,
                    GRAY_1,
                    (0, 0),
                    (200, 30),
                    TAHOMA_BOLD_8_SHADOW_ID,
                    INPUT_ID,
                    context,
                );
                input_field.focused = true;
                self.ui.message_input = Some(input_field);
            }
            _ => {}
        }
    }

    fn draw_entities<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.draw(context);
        }
    }

    fn draw_map<E: GameEngine>(&mut self, context: &mut Context<E>) {
        // TODO!
        context.engine.draw_image(
            DrawImage {
                position: Position::new(40 * 32, 40 * 32, 0.),
                index: DV_BACKGROUND_ID,
                ..Default::default()
            },
            Target::World,
        );
    }

    fn process_input<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if context.engine.key_pressed(KeyCode::ArrowUp)
            && !self.move_keys.contains(&Direction::North)
        {
            self.move_keys.push_front(Direction::North);
        } else if context.engine.key_pressed(KeyCode::ArrowDown)
            && !self.move_keys.contains(&Direction::South)
        {
            self.move_keys.push_front(Direction::South);
        } else if context.engine.key_pressed(KeyCode::ArrowRight)
            && !self.move_keys.contains(&Direction::East)
        {
            self.move_keys.push_front(Direction::East);
        } else if context.engine.key_pressed(KeyCode::ArrowLeft)
            && !self.move_keys.contains(&Direction::West)
        {
            self.move_keys.push_front(Direction::West);
        } else if context.engine.key_released(KeyCode::ArrowUp) {
            self.move_keys
                .retain(|direction| direction != &Direction::North);
        } else if context.engine.key_released(KeyCode::ArrowDown) {
            self.move_keys
                .retain(|direction| direction != &Direction::South);
        } else if context.engine.key_released(KeyCode::ArrowLeft) {
            self.move_keys
                .retain(|direction| direction != &Direction::West);
        } else if context.engine.key_released(KeyCode::ArrowRight) {
            self.move_keys
                .retain(|direction| direction != &Direction::East);
        }

        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            if let Some(direction) = self.move_keys.front() {
                let distance = context.engine.get_delta().as_secs_f64() * 4. * 32.;
                match direction {
                    Direction::South => character.render_position.1 -= distance,
                    Direction::North => character.render_position.1 += distance,
                    Direction::East => character.render_position.0 += distance,
                    Direction::West => character.render_position.0 -= distance,
                }
                character.animation.change_direction(*direction);
                character
                    .animation
                    .change_animation(CharacterAnimation::Walk);
            } else {
                character
                    .animation
                    .change_animation(CharacterAnimation::Idle);
            }
        }

        if context.engine.key_pressed(KeyCode::KeyH) {
            if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
                character.add_dialog(
                    context.engine,
                    "Rahma Nañarak O'al",
                    TAHOMA_REGULAR_10_ID,
                    BLUE_3,
                );
            }
        }
    }
}

impl GameScreen for WorldScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.process_input(context);

        self.prepare_viewports(context.engine);
        self.ui.update(context);

        self.update_fps(context);
        self.update_ping(context);
        self.update_character(context);
        self.update_message_input(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.ui.draw(context);

        self.draw_map(context);
        self.draw_entities(context);
    }
}

impl WorldUI {
    pub fn initialize<E: GameEngine>(engine: &mut E, character: &Character) -> Self {
        let level = Label::from(
            &character.level.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            GRAY_4,
            engine,
        );

        let mut name = Label::from(&character.name, WIZARD_16_ID, GRAY_4, engine);
        name.alignment = Alignment::Right;

        let mut desc = Label::from(
            &format!("{} {}", character.class, character.race),
            TAHOMA_BOLD_8_SHADOW_ID,
            GRAY_3,
            engine,
        );
        desc.alignment = Alignment::Right;

        let energy_bar = Bar::new(engine, YELLOW);
        let mana_bar = Bar::new(engine, BLUE);
        let health_bar = Bar::new(engine, RED);
        let exp_bar = Bar::with(engine, EXP_BAR_ID, [0, 0, 0, 0]);

        let celerity = Label::from(
            &character.attributes.agility.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            tint(YELLOW, 0.1),
            engine,
        );
        let strength = Label::from(
            &character.attributes.strength.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            tint(GREEN, 0.2),
            engine,
        );
        let mut gold = Label::from(
            &character.gold.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            GRAY_4,
            engine,
        );
        gold.alignment = Alignment::Left;

        let mut fps = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        fps.alignment = Alignment::Right;

        let mut ping = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        ping.alignment = Alignment::Right;

        let mut inventory_button = Button::from(INVENTORY_BUTTON_ID);
        inventory_button.select();
        let inventory = Inventory::initialize(engine);

        let spells_button = Button::from(SPELLS_BUTTON_ID);
        let spellbook = Spellbook::initialize(engine);

        let console = Console::initialize(engine);

        Self {
            x: 0,
            y: 0,

            name,
            desc,
            level,

            exp_bar,
            energy_bar,
            mana_bar,
            health_bar,

            agility: celerity,
            strength,
            gold,

            fps,
            ping,

            inventory_button,
            inventory,

            spells_button,
            spellbook,

            console,
            message_input: None,
        }
    }

    pub fn update_character<E: GameEngine>(
        &mut self,
        context: &mut Context<E>,
        character: &Character,
    ) {
        let stats = &character.stats;
        self.energy_bar
            .set_values(context.engine, (stats.stamina.current, stats.stamina.max));
        self.health_bar
            .set_values(context.engine, (stats.health.current, stats.health.max));
        self.mana_bar
            .set_values(context.engine, (stats.mana.current, stats.mana.max));
        self.exp_bar
            .set_values(context.engine, (character.exp.current, character.exp.max));
        self.strength
            .set_text(&character.attributes.strength.to_string(), context.engine);
        self.agility
            .set_text(&character.attributes.agility.to_string(), context.engine);
        let gold = character
            .gold
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(".");
        self.gold.set_text(&gold, context.engine);
    }

    fn recalculate_positions(&mut self) {
        let right_panel_x_start = self.x + 14 + WORLD_RENDER_WIDTH;
        self.level.position = (right_panel_x_start + 42, self.y + SCREEN_HEIGHT - 24);
        self.name.position = (self.x + SCREEN_WIDTH - 30, self.y + SCREEN_HEIGHT - 18);
        self.desc.position = (self.x + SCREEN_WIDTH - 30, self.y + SCREEN_HEIGHT - 32);
        self.exp_bar
            .set_position(right_panel_x_start + 120, self.y + SCREEN_HEIGHT - 25);

        self.inventory_button.position =
            (right_panel_x_start + 10 + 53, self.y + SCREEN_HEIGHT - 75);
        self.inventory.position = (right_panel_x_start + 22, self.y + SCREEN_HEIGHT - 76);
        self.spells_button.position = (
            right_panel_x_start + 20 + 100 + 53,
            self.y + SCREEN_HEIGHT - 75,
        );
        self.spellbook.position = (right_panel_x_start + 22, self.y + SCREEN_HEIGHT - 76);

        let bars_x = right_panel_x_start + 130;
        self.energy_bar.set_position(bars_x, self.y + 193);
        self.health_bar.set_position(bars_x, self.y + 164);
        self.mana_bar.set_position(bars_x, self.y + 135);

        self.agility.position = (self.x + WORLD_RENDER_WIDTH + 167, self.y + 229);
        self.strength.position = (self.x + WORLD_RENDER_WIDTH + 207, self.y + 229);
        self.gold.position = (self.x + WORLD_RENDER_WIDTH + 60, self.y + 229);
        self.fps.position = (self.x + SCREEN_WIDTH - 100, self.y + 32);
        self.ping.position = (self.x + SCREEN_WIDTH - 50, self.y + 32);

        self.console.position = (self.x + 20, self.y + 10 + WORLD_RENDER_HEIGHT);

        if let Some(input) = self.message_input.as_mut() {
            let x = self.x + 14 + WORLD_RENDER_WIDTH / 2;
            let y = self.y + 20;
            input.position = (x, y);
        }
    }
}

impl UI for WorldUI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        // recalculate positions (we can avoid doing every frame)
        self.recalculate_positions();

        // updates
        self.inventory_button.update(context);
        self.spells_button.update(context);
        if self.inventory_button.clicked() {
            self.inventory_button.select();
            self.inventory.show();
            self.spells_button.unselect();
            self.spellbook.hide();
        } else if self.spells_button.clicked() {
            self.inventory_button.unselect();
            self.inventory.hide();
            self.spells_button.select();
            self.spellbook.show();
        }

        self.inventory.update(context);
        self.spellbook.update(context);

        self.exp_bar.update(context);
        self.energy_bar.update(context);
        self.health_bar.update(context);
        self.mana_bar.update(context);

        self.console.update(context);

        if let Some(input) = self.message_input.as_mut() {
            input.update(context)
        }
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        context.engine.draw_image(
            DrawImage {
                position: Position::new(self.x, self.y, 0.),
                index: MAIN_UI_ID,
                ..Default::default()
            },
            Target::UI,
        );

        self.exp_bar.draw(context);
        self.level.draw(context);
        self.desc.draw(context);
        self.name.draw(context);

        self.inventory_button.draw(context);
        self.inventory.draw(context);

        self.spells_button.draw(context);
        self.spellbook.draw(context);

        self.strength.draw(context);
        self.agility.draw(context);
        self.gold.draw(context);

        self.energy_bar.draw(context);
        self.health_bar.draw(context);
        self.mana_bar.draw(context);

        self.fps.draw(context);
        self.ping.draw(context);

        self.console.draw(context);

        if let Some(input) = self.message_input.as_mut() {
            input.draw(context)
        }
    }
}
