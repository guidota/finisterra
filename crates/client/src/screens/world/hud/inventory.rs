use engine::engine::GameEngine;

use crate::game::Context;

use crate::ui::{
    colors::*, fonts::TAHOMA_BOLD_8_SHADOW_ID, image::Image, label::Label, Alignment, Widget,
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
    pub fn initialize<E: GameEngine>(context: &mut Context<E>) -> Self {
        let background = Image::new(context.resources.textures.inventory_list, WHITE, (0, 0));

        let mut slots = vec![];
        for _ in 0..(GRID_SIZE * GRID_SIZE) {
            slots.push(InventorySlot::initialize(context));
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
                self.position.0 + 6 + column as u16 * SLOT_SIZE,
                self.position.1 - 6 - SLOT_SIZE - row as u16 * SLOT_SIZE,
            );
            slot.update(context);
        }

        // if click => select
        // if dragging => swap on drop
        // if double clicking => use slot
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
    pub fn initialize<E: GameEngine>(context: &mut Context<E>) -> Self {
        let mut color = GRAY_2;
        color[3] = 0;

        let background = Image::new(context.resources.textures.inventory_slot, color, (0, 0));
        let amount = Label::from("9999", TAHOMA_BOLD_8_SHADOW_ID, GRAY_4, context.engine);
        let equipped = Label::from("+", TAHOMA_BOLD_8_SHADOW_ID, YELLOW, context.engine);
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
            self.position.0 + SLOT_SIZE - 3,
            self.position.1 + self.amount.parsed_text.height - 3,
        );
        self.amount.alignment = Alignment::Right;

        self.equipped.position = (
            self.position.0 + SLOT_SIZE - 3,
            self.position.1 + SLOT_SIZE - 4,
        );
        self.equipped.alignment = Alignment::Right;
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
