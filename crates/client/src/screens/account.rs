use engine::draw::image::DrawImage;
use engine::draw::Position;
use engine::draw::Target;
use engine::engine::GameEngine;
use protocol::character;
use protocol::client;
use protocol::client::ClientPacket;
use protocol::server;
use protocol::server::ServerPacket;
use tracing::info;

use crate::game::Context;
use crate::ui::button::Button;
use crate::ui::button::ButtonBuilder;
use crate::ui::colors::*;
use crate::ui::fonts::*;
use crate::ui::label::Label;
use crate::ui::textures::*;
use crate::ui::Widget;
use crate::ui::UI;

use super::character_creation::CharacterCreationScreen;
use super::prepare_viewport;
use super::screen_size;
use super::world::entity;
use super::world::entity::Character;
use super::world::WorldScreen;
use super::GameScreen;
use super::Screen;

const SLOTS: usize = 6;
const SLOT_SIZE: u16 = 64;
const SPACING: u16 = 30;

pub struct AccountScreen {
    ui: AccountUI,

    connecting: bool,
}

pub struct AccountUI {
    slots: [Slot; SLOTS],
    selected: Option<usize>,
    enter_button: Button,
}

pub enum Slot {
    Char {
        button: Button,
        // label: Label,
        character: Box<Character>,
    },
    Empty {
        button: Button,
    },
}

impl AccountScreen {
    pub fn new<E: GameEngine>(
        context: &mut Context<E>,
        characters: Vec<character::CharacterPreview>,
    ) -> Self {
        Self {
            ui: AccountUI::initialize(context, characters),
            connecting: false,
        }
    }
}

impl GameScreen for AccountScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        prepare_viewport(context);

        self.ui.update(context);

        for slot in &self.ui.slots {
            if let Slot::Empty { button } = slot {
                if button.clicked() {
                    context
                        .screen_transition_sender
                        .send(Screen::CharacterCreation(Box::new(
                            CharacterCreationScreen::new(context),
                        )))
                        .expect("poisoned");
                    return;
                }
            }
        }

        let messages = context.connection.read();
        if !self.connecting && self.ui.enter_button.clicked() && self.ui.selected.is_some() {
            let slot = &self.ui.slots[self.ui.selected.unwrap()];
            if let Slot::Char { character, .. } = slot {
                self.connecting = true;
                context
                    .connection
                    .send(ClientPacket::Account(client::Account::LoginCharacter {
                        character: character.name.to_string(),
                    }));
            }
        } else {
            for message in messages {
                match message {
                    ServerPacket::Account(server::Account::LoginCharacterOk { character }) => {
                        let character = entity::Character::from(context, character);
                        context
                            .screen_transition_sender
                            .send(Screen::World(Box::new(WorldScreen::new(
                                context.engine,
                                0,
                                character,
                            ))))
                            .expect("poisoned")
                    }
                    ServerPacket::Account(server::Account::LoginCharacterFailed { reason }) => {
                        info!("login character failed {reason}");
                        self.connecting = false;
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.ui.draw(context);
    }
}

impl AccountUI {
    fn initialize<E: GameEngine>(
        context: &mut Context<E>,
        characters: Vec<character::CharacterPreview>,
    ) -> Self {
        let mut button = |character: &character::CharacterPreview| Slot::Char {
            button: ButtonBuilder::new()
                .texture_id(CHAR_SLOT_ID)
                .size((SLOT_SIZE, SLOT_SIZE))
                .color(GRAY_1)
                .selected_color(GREEN)
                .build(),

            character: Box::new(Character::from_preview(context, character.clone())),
        };
        let empty = || Slot::Empty {
            button: ButtonBuilder::new()
                .texture_id(NEW_CHAR_SLOT_ID)
                .size((SLOT_SIZE, SLOT_SIZE))
                .color(GRAY_2)
                .build(),
        };
        let slots = [
            characters.first().map(&mut button).unwrap_or(empty()),
            characters.get(1).map(&mut button).unwrap_or(empty()),
            characters.get(2).map(&mut button).unwrap_or(empty()),
            characters.get(3).map(&mut button).unwrap_or(empty()),
            characters.get(4).map(&mut button).unwrap_or(empty()),
            characters.get(5).map(&mut button).unwrap_or(empty()),
        ];

        let enter_label = Label::from("Enter", TAHOMA_BOLD_8_SHADOW_ID, GRAY_6, context.engine);
        let enter_button = ButtonBuilder::new()
            .color(GRAY_2)
            .label(enter_label)
            .texture_id(BUTTON_ID)
            .build();

        Self {
            slots,
            selected: None,
            enter_button,
        }
    }
}

impl UI for AccountUI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let size = screen_size(context.engine);
        let all_slots_width = (SLOT_SIZE + SPACING) * SLOTS as u16;
        let center_x = size.0 / 2;
        let mut x = center_x - all_slots_width / 2 + 32;
        let center_y = size.1 / 2;

        for (i, slot) in self.slots.iter_mut().enumerate() {
            slot.button().position = (x, center_y);
            slot.button().update(context);

            if let Slot::Char { character, .. } = slot {
                character.render_position.0 = x;
                character.render_position.1 = center_y + 2;
                if slot.button().clicked() {
                    slot.button().select();
                    self.selected = Some(i);
                } else if slot.button().selected() {
                    if let Some(selected) = self.selected {
                        if selected != i {
                            slot.button().unselect();
                        }
                    }
                }
            }
            x += SLOT_SIZE + SPACING;
        }
        self.enter_button.update(context);
        self.enter_button.position = (center_x, center_y - 100);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        context.engine.draw_image(
            DrawImage {
                position: Position { x: 0, y: 0, z: 0. },
                color: WHITE,
                index: DV_BACKGROUND_ID,
                source: [0, 0, 0, 0],
            },
            Target::UI,
        );
        for slot in self.slots.iter_mut() {
            slot.button().draw(context);
            if let Slot::Char { character, .. } = slot {
                character.draw(context);
            }
        }
        self.enter_button.draw(context);
    }
}

impl Slot {
    fn button(&mut self) -> &mut Button {
        match self {
            Slot::Char { button, .. } => button,
            Slot::Empty { button } => button,
        }
    }
}
