use engine::{
    draw::Position,
    draw::{image::DrawImage, Target},
    engine::GameEngine,
};

use crate::{
    game::Context,
    ui::{
        bar::Bar,
        button::{Button, ButtonBuilder},
        colors::*,
        fonts::*,
        input_field::InputField,
        label::Label,
        Alignment, Widget, UI,
    },
};

use self::{console::Console, inventory::Inventory, spellbook::Spellbook};

use super::{
    entity::Character, SCREEN_HEIGHT, SCREEN_WIDTH, WORLD_RENDER_HEIGHT, WORLD_RENDER_WIDTH,
};

pub mod console;
pub mod inventory;
pub mod spellbook;

pub struct HUD {
    pub x: u16,
    pub y: u16,

    // header
    pub name: Label,
    pub desc: Label,

    // inventory
    pub inventory_button: Button,
    pub inventory: Inventory,

    // spellbook
    pub spells_button: Button,
    pub spellbook: Spellbook,

    // stats
    pub exp_bar: Bar,
    pub energy_bar: Bar,
    pub mana_bar: Bar,
    pub health_bar: Bar,
    pub agility: Label,
    pub strength: Label,
    pub gold: Label,

    // info
    pub fps: Label,
    pub ping: Label,

    pub console: Console,
    pub message_input: Option<InputField>,
}

impl HUD {
    pub fn initialize<E: GameEngine>(context: &mut Context<E>, character: &Character) -> Self {
        let mut name = Label::from(&character.name, WIZARD_16_ID, GRAY_4, context.engine);
        name.alignment = Alignment::Center;

        let mut desc = Label::from(
            &format!("{} - Lv. {}", character.class, character.level),
            TAHOMA_BOLD_8_SHADOW_ID,
            GRAY_3,
            context.engine,
        );
        desc.alignment = Alignment::Center;

        let energy_bar = Bar::new(context, YELLOW);
        let mana_bar = Bar::new(context, BLUE);
        let health_bar = Bar::new(context, RED);
        let exp_bar = Bar::with(context.engine, context.resources.textures.exp, [0, 0, 0, 0]);

        let celerity = Label::from(
            &character.attributes.agility.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            tint(YELLOW, 0.1),
            context.engine,
        );
        let strength = Label::from(
            &character.attributes.strength.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            tint(GREEN, 0.2),
            context.engine,
        );
        let mut gold = Label::from(
            &character.gold.to_string(),
            TAHOMA_BOLD_8_SHADOW_ID,
            GRAY_4,
            context.engine,
        );
        gold.alignment = Alignment::Left;

        let mut fps = Label::from(
            "0",
            TAHOMA_BOLD_8_SHADOW_ID,
            transparent(GRAY_4, 128),
            context.engine,
        );
        fps.alignment = Alignment::Right;

        let mut ping = Label::from(
            "0",
            TAHOMA_BOLD_8_SHADOW_ID,
            transparent(GRAY_4, 128),
            context.engine,
        );
        ping.alignment = Alignment::Right;

        let mut inventory_button = ButtonBuilder::new()
            .texture_id(context.resources.textures.inventory_button_disabled)
            .selected_texture(context.resources.textures.inventory_button)
            .z(0.999)
            .color(GRAY_5)
            .alignment(Alignment::Right)
            .target(Target::UI)
            .build();
        inventory_button.select();
        let inventory = Inventory::initialize(context);

        let spells_button = ButtonBuilder::new()
            .texture_id(context.resources.textures.spells_button_disabled)
            .selected_texture(context.resources.textures.spells_button)
            .z(0.999)
            .alignment(Alignment::Left)
            .color(GRAY_5)
            .target(Target::UI)
            .build();
        let spellbook = Spellbook::initialize(context);

        let console = Console::initialize(context.engine);

        Self {
            x: 0,
            y: 0,

            name,
            desc,

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
        let right_panel_x_start = self.x + 12 + WORLD_RENDER_WIDTH;
        self.name.position = (self.x + SCREEN_WIDTH - 120, self.y + 35);
        self.desc.position = (self.x + SCREEN_WIDTH - 120, self.y + 20);
        self.exp_bar
            .set_position(right_panel_x_start + 120, self.y + SCREEN_HEIGHT - 25);

        self.inventory_button.position = (right_panel_x_start + 121, self.y + SCREEN_HEIGHT - 48);
        self.inventory.position = (right_panel_x_start + 22, self.y + SCREEN_HEIGHT - 48);
        self.spells_button.position = (right_panel_x_start + 121, self.y + SCREEN_HEIGHT - 48);
        self.spellbook.position = (right_panel_x_start + 12, self.y + SCREEN_HEIGHT - 48);

        let bars_x = right_panel_x_start + 130;
        self.energy_bar.set_position(bars_x, self.y + 179);
        self.health_bar.set_position(bars_x, self.y + 150);
        self.mana_bar.set_position(bars_x, self.y + 121);

        self.agility.position = (self.x + WORLD_RENDER_WIDTH + 167, self.y + 221);
        self.strength.position = (self.x + WORLD_RENDER_WIDTH + 207, self.y + 221);
        self.gold.position = (self.x + WORLD_RENDER_WIDTH + 60, self.y + 221);
        self.fps.position = (
            self.x + WORLD_RENDER_WIDTH - 10,
            self.y + WORLD_RENDER_HEIGHT - 23,
        );
        self.ping.position = (
            self.x + WORLD_RENDER_WIDTH - 10,
            self.y + WORLD_RENDER_HEIGHT - 10,
        );

        self.console.position = (self.x + 20, self.y + 10 + WORLD_RENDER_HEIGHT);

        if let Some(input) = self.message_input.as_mut() {
            let x = self.x + 14 + WORLD_RENDER_WIDTH / 2;
            let y = self.y + 20;
            input.position = (x, y);
        }
    }
}

impl UI for HUD {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        // recalculate positions (we can avoid doing every frame)
        self.recalculate_positions();

        self.inventory_button.update(context);
        self.spells_button.update(context);

        if !self.spellbook.is_dragging() {
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
                position: Position::new(self.x, self.y, 0.99),
                index: context.resources.textures.main,
                ..Default::default()
            },
            Target::UI,
        );

        self.exp_bar.draw(context);
        self.desc.draw(context);
        self.name.draw(context);

        self.inventory_button.draw(context);
        self.spells_button.draw(context);

        self.inventory.draw(context);
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

        context.engine.draw_image(
            DrawImage {
                position: Position::new(self.x + 565, self.y, 1.),
                index: context.resources.textures.main_frame,
                ..Default::default()
            },
            Target::UI,
        );
    }
}
