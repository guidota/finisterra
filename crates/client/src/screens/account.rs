use engine::draw::image::DrawImage;
use engine::draw::Position;
use engine::draw::Target;
use engine::engine::GameEngine;
use protocol::client;
use protocol::client::ClientPacket;
use protocol::server;
use protocol::server::ServerPacket;
use tracing::info;

use crate::ui::button::Button;
use crate::ui::button::ButtonBuilder;
use crate::ui::colors::*;
use crate::ui::fonts::*;
use crate::ui::label::Label;
use crate::ui::textures::*;
use crate::ui::Alignment;
use crate::ui::Widget;
use crate::ui::UI;

use super::character_creation::CharacterCreationScreen;
use super::prepare_viewport;
use super::screen_size;
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
        label: Label,
        character: String,
    },
    Empty {
        button: Button,
    },
}

impl AccountScreen {
    pub fn new<E: GameEngine>(engine: &mut E, characters: Vec<String>) -> Self {
        Self {
            ui: AccountUI::initialize(engine, characters),
            connecting: false,
        }
    }
}

impl GameScreen for AccountScreen {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        prepare_viewport(context);

        self.ui.update(context.engine);

        for slot in &self.ui.slots {
            if let Slot::Empty { button } = slot {
                if button.clicked() {
                    context
                        .screen_transition_sender
                        .send(Screen::CharacterCreation(Box::new(
                            CharacterCreationScreen::new(context.engine),
                        )))
                        .expect("poisoned")
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
                        character: character.to_string(),
                    }));
            }
        } else {
            for message in messages {
                match message {
                    ServerPacket::Account(server::Account::LoginCharacterOk { .. }) => context
                        .screen_transition_sender
                        .send(Screen::World(Box::new(WorldScreen::new(context.engine))))
                        .expect("poisoned"),
                    ServerPacket::Account(server::Account::LoginCharacterFailed { reason }) => {
                        info!("login character failed {reason}");
                        self.connecting = false;
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        self.ui.draw(context.engine);
    }
}

impl AccountUI {
    fn initialize<E: GameEngine>(engine: &mut E, characters: Vec<String>) -> Self {
        let mut button = |text: &str| Slot::Char {
            button: ButtonBuilder::new()
                .texture_id(CHAR_SLOT_ID)
                .size((SLOT_SIZE, SLOT_SIZE))
                .color(GRAY_4)
                .build(),
            label: Label::new(
                engine.parse_text(TAHOMA_BOLD_8_SHADOW_ID, text).unwrap(),
                TAHOMA_BOLD_8_SHADOW_ID,
                BLUE,
            ),
            character: text.to_string(),
        };
        let empty = || Slot::Empty {
            button: ButtonBuilder::new()
                .texture_id(NEW_CHAR_SLOT_ID)
                .size((SLOT_SIZE, SLOT_SIZE))
                .color(GRAY_4)
                .build(),
        };
        let slots = [
            characters
                .first()
                .map(|char| button(char))
                .unwrap_or(empty()),
            characters
                .get(1)
                .map(|char| button(char))
                .unwrap_or(empty()),
            characters
                .get(2)
                .map(|char| button(char))
                .unwrap_or(empty()),
            characters
                .get(3)
                .map(|char| button(char))
                .unwrap_or(empty()),
            characters
                .get(4)
                .map(|char| button(char))
                .unwrap_or(empty()),
            characters
                .get(5)
                .map(|char| button(char))
                .unwrap_or(empty()),
        ];

        let enter_text = engine.parse_text(TAHOMA_BOLD_8_SHADOW_ID, "Enter").unwrap();
        let enter_label = Label {
            text: enter_text,
            position: (0, 0),
            color: GRAY_6,
            texture_id: TAHOMA_BOLD_8_SHADOW_ID,
            alignment: Alignment::Center,
        };
        let enter_button = Button {
            position: (0, 0),
            size: (80, 20),
            color: GRAY_2,
            texture_id: BUTTON_ID,
            label: Some(enter_label),
            ..Default::default()
        };

        Self {
            slots,
            selected: None,
            enter_button,
        }
    }
}

impl UI for AccountUI {
    fn update<E: GameEngine>(&mut self, engine: &mut E) {
        let size = screen_size(engine);
        let all_slots_width = (SLOT_SIZE + SPACING) * SLOTS as u16;
        let center_x = size.0 / 2;
        let mut x = center_x - all_slots_width / 2 + 32;
        let center_y = size.1 / 2;

        for (i, slot) in self.slots.iter_mut().enumerate() {
            slot.button().position = (x, center_y);
            slot.button().update(engine);

            if let Slot::Char { label, .. } = slot {
                label.position = (x + 2, center_y - 20);
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
        self.enter_button.update(engine);
        self.enter_button.position = (center_x, center_y - 100);
    }

    fn draw<E: GameEngine>(&mut self, engine: &mut E) {
        engine.draw_image(
            DrawImage {
                position: Position { x: 0, y: 0, z: 0. },
                color: WHITE,
                index: DV_BACKGROUND_ID,
                source: [0, 0, 0, 0],
            },
            Target::UI,
        );
        for slot in self.slots.iter_mut() {
            slot.button().draw(engine);
            if let Slot::Char { label, .. } = slot {
                label.draw(engine);
            }
        }
        self.enter_button.draw(engine);
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
