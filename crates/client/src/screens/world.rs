use std::time::Duration;

use engine::{
    camera::{self, Viewport, Zoom},
    draw::{image::DrawImage, Color, Position, Target},
    engine::{GameEngine, TextureID},
};
use interpolation::lerp;
use nohash_hasher::IntMap;
use tracing::info;

use crate::{
    game::Context,
    ui::{
        button::Button,
        colors::*,
        fonts::{TAHOMA_BOLD_8_SHADOW_ID, WIZARD_16_ID},
        image::Image,
        label::Label,
        textures::{
            BAR_ID, DV_BACKGROUND_ID, EXP_BAR_ID, INVENTORY_BUTTON_ID, MAIN_UI_ID, SPELLS_BUTTON_ID,
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
    width: u16,
    height: u16,

    inventory_button: Button,
    spells_button: Button,

    level: Label,
    name: Label,
    desc: Label,

    exp_bar: Bar,
    energy_bar: Bar,
    mana_bar: Bar,
    health_bar: Bar,

    celerity: Label,
    strength: Label,
    gold: Label,

    fps: Label,
    ping: Label,
}

struct Bar {
    label: Label,
    image: Image,

    min: u16,
    max: u16,

    target: u16,
    interpolation_time: Duration,
}

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 540;

const TILE_SIZE: u16 = 32;
const WORLD_RENDER_WIDTH: u16 = 17 * TILE_SIZE; // 17 tiles
const WORLD_RENDER_HEIGHT: u16 = 16 * TILE_SIZE; // 16 tiles

impl WorldScreen {
    pub fn new<E: GameEngine>(engine: &mut E, entity_id: u32, character: Character) -> Self {
        let mut entities = IntMap::default();
        entities.insert(entity_id, Entity::Character(character));

        Self {
            ui: WorldUI::initialize(engine),
            entities,
            me: entity_id,
        }
    }
}

impl GameScreen for WorldScreen {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut Context<E>) {
        self.prepare_viewports(context.engine);
        context
            .engine
            .set_world_camera_position(camera::Position { x: 300., y: 300. });

        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.update(context.engine);
        }

        self.ui.update(context.engine);
        self.ui.energy_bar.set_values(context.engine, 500, 999);
        self.ui.health_bar.set_values(context.engine, 600, 999);
        self.ui.mana_bar.set_values(context.engine, 300, 999);
        self.ui.exp_bar.set_values(context.engine, 200, 2000);

        let fps = format!("{:.0}", 1. / context.engine.get_delta().as_secs_f64());
        self.ui.fps.set_text(&fps, context.engine);
        let ping = context.connection.ping();
        self.ui.ping.set_text(&format!("{ping}"), context.engine);
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut Context<E>) {
        self.ui.draw(context.engine);

        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.draw(context.engine, context.resources);
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
        self.ui.width = SCREEN_WIDTH;
        self.ui.height = SCREEN_HEIGHT;

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
    pub fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        let level = Label::from("33", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        let name = Label::from("Pandora", WIZARD_16_ID, GRAY_4, engine);
        let desc = Label::from("Clerigo Humano", TAHOMA_BOLD_8_SHADOW_ID, GRAY_3, engine);

        let energy_bar = Bar::new(engine, YELLOW);
        let mana_bar = Bar::new(engine, BLUE);
        let health_bar = Bar::new(engine, RED);
        let exp_bar = Bar::with(engine, EXP_BAR_ID);

        let celerity = Label::from("18", TAHOMA_BOLD_8_SHADOW_ID, tint(YELLOW, 0.1), engine);
        let strength = Label::from("18", TAHOMA_BOLD_8_SHADOW_ID, tint(GREEN, 0.2), engine);
        let gold = Label::from("100.000", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);

        let fps = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);
        let ping = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, engine);

        let inventory_button = Button::from(INVENTORY_BUTTON_ID);
        let spells_button = Button::from(SPELLS_BUTTON_ID);

        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,

            name,
            desc,
            level,

            exp_bar,
            energy_bar,
            mana_bar,
            health_bar,

            celerity,
            strength,
            gold,

            fps,
            ping,

            inventory_button,
            spells_button,
        }
    }
}

impl Bar {
    pub fn new<E: GameEngine>(engine: &mut E, color: Color) -> Self {
        let label = Label {
            text: engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "999/999")
                .expect("can parse"),
            position: (0, 0),
            // color: tint(color, 0.5),
            color: GRAY_6,
            texture_id: TAHOMA_BOLD_8_SHADOW_ID,
            alignment: Alignment::Center,
        };

        let mut image = Image::new(BAR_ID, color, (0, 0));
        image.percent = 100;
        Self {
            label,
            image,
            min: 100,
            max: 100,
            target: 100,
            interpolation_time: Duration::ZERO,
        }
    }

    pub fn with<E: GameEngine>(engine: &mut E, texture_id: TextureID) -> Self {
        let label = Label {
            text: engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "")
                .expect("can parse"),
            position: (0, 0),
            // color: tint(color, 0.5),
            color: [255, 255, 255, 0],
            texture_id: TAHOMA_BOLD_8_SHADOW_ID,
            alignment: Alignment::Center,
        };

        let mut image = Image::new(texture_id, [255, 255, 255, 120], (0, 0));
        image.percent = 100;
        Self {
            label,
            image,
            min: 100,
            max: 100,
            target: 100,
            interpolation_time: Duration::ZERO,
        }
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.label.position = (x, y);
        self.image.position = (x, y);
    }

    pub fn set_values<E: GameEngine>(&mut self, engine: &mut E, min: u16, max: u16) {
        if min != self.min || max != self.max {
            self.min = min;
            self.max = max;
            self.label.text = engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format! {"{min}/{max}"})
                .expect("can parse");

            let percent = ((min as f32 / max as f32) * 100.) as u16;
            self.target = percent;
            self.interpolation_time = Duration::ZERO;
        }
    }

    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        self.image.update(engine);
        const INTERPOLIATION_DURATION: Duration = Duration::from_millis(250);
        if self.target != self.image.percent && self.interpolation_time < INTERPOLIATION_DURATION {
            let delta = engine.get_delta();
            self.interpolation_time += delta;

            let time_percent = self.interpolation_time.as_millis() as f32
                / INTERPOLIATION_DURATION.as_millis() as f32;
            let percent = lerp(&self.image.percent, &self.target, &time_percent);
            self.image.set_percent(percent);
        }
    }

    pub fn draw<E: GameEngine>(&mut self, engine: &mut E) {
        self.image.draw(engine);
        self.label.draw(engine);
    }
}

impl UI for WorldUI {
    fn update<E: engine::engine::GameEngine>(&mut self, engine: &mut E) {
        self.level.position = (
            self.x + 14 + WORLD_RENDER_WIDTH + 43,
            self.y + SCREEN_HEIGHT - 24,
        );
        self.name.position = (self.x + SCREEN_WIDTH - 65, self.y + SCREEN_HEIGHT - 18);
        self.desc.position = (self.x + SCREEN_WIDTH - 77, self.y + SCREEN_HEIGHT - 32);

        self.inventory_button.position = (
            self.x + 14 + WORLD_RENDER_WIDTH + 10 + 53,
            self.y + SCREEN_HEIGHT - 80,
        );
        self.spells_button.position = (
            self.x + 14 + WORLD_RENDER_WIDTH + 20 + 100 + 53,
            self.y + SCREEN_HEIGHT - 80,
        );
        self.inventory_button.update(engine);
        self.spells_button.update(engine);

        self.exp_bar.update(engine);
        self.exp_bar.set_position(
            self.x + 14 + WORLD_RENDER_WIDTH + 120,
            self.y + SCREEN_HEIGHT - 25,
        );

        let bars_x = self.x + WORLD_RENDER_WIDTH + 144; // 40?
        self.energy_bar.update(engine);
        self.health_bar.update(engine);
        self.mana_bar.update(engine);

        self.energy_bar.set_position(bars_x, self.y + 193);
        self.health_bar.set_position(bars_x, self.y + 164);
        self.mana_bar.set_position(bars_x, self.y + 135);

        self.celerity.position = (self.x + WORLD_RENDER_WIDTH + 167, self.y + 229);
        self.strength.position = (self.x + WORLD_RENDER_WIDTH + 207, self.y + 229);
        self.gold.position = (self.x + WORLD_RENDER_WIDTH + 82, self.y + 229);

        self.fps.position = (self.x + SCREEN_WIDTH - 115, self.y + 32);
        self.ping.position = (self.x + SCREEN_WIDTH - 62, self.y + 32);
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, engine: &mut E) {
        engine.draw_image(
            DrawImage {
                position: Position::new(0, 0, 0.),
                index: DV_BACKGROUND_ID,
                ..Default::default()
            },
            Target::UI,
        );

        engine.draw_image(
            DrawImage {
                position: Position::new(self.x, self.y, 1.),
                index: MAIN_UI_ID,
                ..Default::default()
            },
            Target::UI,
        );

        self.exp_bar.draw(engine);

        self.level.draw(engine);
        self.desc.draw(engine);
        self.name.draw(engine);

        self.energy_bar.draw(engine);
        self.health_bar.draw(engine);
        self.mana_bar.draw(engine);
        self.strength.draw(engine);
        self.celerity.draw(engine);
        self.gold.draw(engine);
        self.fps.draw(engine);
        self.ping.draw(engine);

        self.inventory_button.draw(engine);
        self.spells_button.draw(engine);
    }
}
