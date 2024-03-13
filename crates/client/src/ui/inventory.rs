use engine::engine::GameEngine;

use super::{
    colors::*, fonts::TAHOMA_BOLD_8_SHADOW_ID, image::Image, label::Label, textures::*, Alignment,
    Widget,
};

pub struct Inventory {
    background: Image,
    slots: Vec<InventorySlot>,
    pub position: (u16, u16), // top left
    visible: bool,
}

pub struct InventorySlot {
    background: Image,
    item: Option<Image>,
    amount: Label,
    equipped: Label,
    position: (u16, u16),
    is_equipped: bool,
    is_selected: bool,
}

const SLOT_SIZE: u16 = 32;
const GRID_SIZE: usize = 6;

impl Inventory {
    pub fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        // let background = Image::new(INVENTORY_BACKGROUND_ID, shade(GRAY_1, 0.5), (0, 0));
        let background = Image::new(INVENTORY_LIST_ID, WHITE, (0, 0));

        let mut slots = vec![];
        for _ in 0..(GRID_SIZE * GRID_SIZE) {
            slots.push(InventorySlot::initialize(engine));
        }

        Self {
            background,
            slots,
            position: (0, 0),
            visible: true,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}

impl Widget for Inventory {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        if !self.visible {
            return;
        }
        self.background.update(context);
        self.background.position = (
            self.position.0 + self.background.size.0 / 2,
            self.position.1 - self.background.size.1 / 2,
        );
        for (i, slot) in self.slots.iter_mut().enumerate() {
            let row = i / GRID_SIZE;
            let column = i % GRID_SIZE;

            slot.position = (
                self.position.0 + column as u16 * SLOT_SIZE,
                self.position.1 - SLOT_SIZE - row as u16 * SLOT_SIZE,
            );
            slot.update(context);
        }
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        if !self.visible {
            return;
        }
        self.background.draw(context);
        for slot in self.slots.iter_mut() {
            slot.draw(context);
        }
    }
}

impl InventorySlot {
    pub fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        let mut color = GRAY_2;
        color[3] = 0;

        let background = Image::new(INVENTORY_SLOT_BACKGROUND_ID, color, (0, 0));
        let amount = Label::from("0", TAHOMA_BOLD_8_SHADOW_ID, GRAY_5, engine);
        let equipped = Label::from("+", TAHOMA_BOLD_8_SHADOW_ID, YELLOW, engine);
        let position = (0, 0);
        Self {
            background,
            item: None,
            amount,
            equipped,
            position,
            is_equipped: false,
            is_selected: false,
        }
    }
}

impl Widget for InventorySlot {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        self.background.update(context);
        self.background.position = (
            self.position.0 + SLOT_SIZE / 2,
            self.position.1 + SLOT_SIZE / 2,
        );

        self.amount.position = (
            self.position.0 + SLOT_SIZE,
            self.position.1 + self.amount.parsed_text.height / 2,
        );
        self.amount.alignment = Alignment::Right;

        self.equipped.position = (
            self.position.0 + SLOT_SIZE,
            self.position.1 + SLOT_SIZE - self.equipped.parsed_text.height / 2,
        );
        self.equipped.alignment = Alignment::Right;

        // TODO handle mouse event
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        if self.is_selected {
            self.background.color = RED;
        } else {
            let mut color = GRAY_2;
            color[3] = 0;
            self.background.color = color;
        }
        self.background.draw(context);
        if let Some(item) = self.item.as_mut() {
            item.draw(context);
            self.amount.draw(context);
        }
        if self.is_equipped {
            self.equipped.draw(context);
        }
    }
}
