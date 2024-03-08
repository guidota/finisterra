use engine::{
    camera::{self, Viewport, Zoom},
    draw::{image::DrawImage, Position, Target},
    engine::GameEngine,
};
use nohash_hasher::IntMap;
use tracing::debug;

use crate::{
    game::Context,
    ui::{
        bar::Bar,
        button::Button,
        colors::*,
        fonts::{TAHOMA_BOLD_8_SHADOW_ID, WIZARD_16_ID},
        inventory::Inventory,
        label::Label,
        textures::{
            DV_BACKGROUND_ID, EXP_BAR_ID, INVENTORY_BUTTON_ID, MAIN_UI_ID, SPELLS_BUTTON_ID,
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
}

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 540;

const TILE_SIZE: u16 = 32;
const WORLD_RENDER_WIDTH: u16 = 17 * TILE_SIZE; // 17 tiles
const WORLD_RENDER_HEIGHT: u16 = 16 * TILE_SIZE; // 16 tiles

impl WorldScreen {
    pub fn new<E: GameEngine>(engine: &mut E, entity_id: u32, character: Character) -> Self {
        let ui = WorldUI::initialize(engine, &character);
        let mut entities = IntMap::default();
        entities.insert(entity_id, Entity::Character(character));

        Self {
            ui,
            entities,
            me: entity_id,
        }
    }
}

impl GameScreen for WorldScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.prepare_viewports(context.engine);

        self.ui.update(context);

        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.update(context.engine);
            // camera follows main character
            context.engine.set_world_camera_position(camera::Position {
                x: character.render_position.0 as f32,
                y: character.render_position.1 as f32,
            });

            self.ui.update_char(context, character);
        }

        let fps = format!("{:.0}", 1. / context.engine.get_delta().as_secs_f64());
        self.ui.fps.set_text(&fps, context.engine);
        let ping = context.connection.ping();
        self.ui.ping.set_text(&format!("{ping}"), context.engine);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.ui.draw(context);

        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.draw(context);
        }
    }
}

impl WorldScreen {
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

        let celerity = Label::from("18", TAHOMA_BOLD_8_SHADOW_ID, tint(YELLOW, 0.1), engine);
        let strength = Label::from("18", TAHOMA_BOLD_8_SHADOW_ID, tint(GREEN, 0.2), engine);
        let mut gold = Label::from("100.000", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        gold.alignment = Alignment::Left;

        let mut fps = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        fps.alignment = Alignment::Right;

        let mut ping = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        ping.alignment = Alignment::Right;

        let mut inventory_button = Button::from(INVENTORY_BUTTON_ID);
        inventory_button.select();
        let inventory = Inventory::initialize(engine);

        let spells_button = Button::from(SPELLS_BUTTON_ID);

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
        }
    }

    pub fn update_char<E: GameEngine>(&mut self, context: &mut Context<E>, character: &Character) {
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
}

impl UI for WorldUI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        // recalculate positions (we can avoid doing every frame)
        let right_panel_x_start = self.x + 14 + WORLD_RENDER_WIDTH;
        self.level.position = (right_panel_x_start + 42, self.y + SCREEN_HEIGHT - 24);
        self.name.position = (self.x + SCREEN_WIDTH - 30, self.y + SCREEN_HEIGHT - 18);
        self.desc.position = (self.x + SCREEN_WIDTH - 30, self.y + SCREEN_HEIGHT - 32);
        self.exp_bar
            .set_position(right_panel_x_start + 120, self.y + SCREEN_HEIGHT - 25);

        self.inventory_button.position =
            (right_panel_x_start + 10 + 53, self.y + SCREEN_HEIGHT - 80);
        self.inventory.position = (right_panel_x_start + 40, self.y + SCREEN_HEIGHT - 80);
        self.spells_button.position = (
            right_panel_x_start + 20 + 100 + 53,
            self.y + SCREEN_HEIGHT - 80,
        );

        let bars_x = right_panel_x_start + 130;
        self.energy_bar.set_position(bars_x, self.y + 193);
        self.health_bar.set_position(bars_x, self.y + 164);
        self.mana_bar.set_position(bars_x, self.y + 135);

        self.agility.position = (self.x + WORLD_RENDER_WIDTH + 167, self.y + 229);
        self.strength.position = (self.x + WORLD_RENDER_WIDTH + 207, self.y + 229);
        self.gold.position = (self.x + WORLD_RENDER_WIDTH + 60, self.y + 229);
        self.fps.position = (self.x + SCREEN_WIDTH - 100, self.y + 32);
        self.ping.position = (self.x + SCREEN_WIDTH - 50, self.y + 32);

        // updates
        self.inventory_button.update(context);
        if self.inventory_button.clicked() {
            self.inventory_button.select();
            self.inventory.show();
            self.spells_button.unselect();
            // TODO! hide spells
        } else if self.spells_button.clicked() {
            self.inventory_button.unselect();
            self.inventory.hide();
            self.spells_button.select();
            // TODO! show spells
        }

        self.inventory.update(context);

        self.spells_button.update(context);

        self.exp_bar.update(context);
        self.energy_bar.update(context);
        self.health_bar.update(context);
        self.mana_bar.update(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        context.engine.draw_image(
            DrawImage {
                position: Position::new(0, 0, 0.),
                index: DV_BACKGROUND_ID,
                ..Default::default()
            },
            Target::UI,
        );

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

        self.strength.draw(context);
        self.agility.draw(context);
        self.gold.draw(context);

        self.energy_bar.draw(context);
        self.health_bar.draw(context);
        self.mana_bar.draw(context);

        self.fps.draw(context);
        self.ping.draw(context);
    }
}
