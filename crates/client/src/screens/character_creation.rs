use engine::{
    draw::{image::DrawImage, Position, Target},
    engine::GameEngine,
};
use protocol::{client, server};
use tracing::info;

use crate::{
    game::Context,
    ui::{
        button::Button, colors::*, fonts::*, input_field::InputField, label::Label, textures::*,
        Alignment, Widget, UI,
    },
};

use super::{
    prepare_viewport, screen_size,
    world::{entity, WorldScreen},
    GameScreen, Screen,
};

pub struct CharacterCreationScreen {
    ui: CharacterCreationUI,
    creating: bool,
}

pub struct CharacterCreationUI {
    name_label: Label,
    pub name_input: InputField,

    pub create_button: Button,
}

impl CharacterCreationScreen {
    pub fn new<E: GameEngine>(context: &mut Context<E>) -> Self {
        let ui = CharacterCreationUI::initialize(context);
        Self {
            ui,
            creating: false,
        }
    }
}

impl GameScreen for CharacterCreationScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        prepare_viewport(context);

        self.ui.update(context);

        let messages = context.connection.read();
        if !self.creating && self.ui.create_button.clicked() {
            self.creating = true;
            let name = self.ui.name_input.text();
            context
                .connection
                .send(protocol::client::ClientPacket::Account(
                    client::Account::CreateCharacter {
                        name: name.to_string(),
                    },
                ))
        } else {
            for message in messages {
                match message {
                    protocol::server::ServerPacket::Account(
                        server::Account::CreateCharacterOk { character },
                    ) => {
                        let character = entity::Character::from(context, &character);

                        context
                            .screen_transition_sender
                            .send(Screen::World(Box::new(WorldScreen::new(
                                context.engine,
                                0,
                                character,
                            ))))
                            .expect("poisoned")
                    }
                    protocol::server::ServerPacket::Account(
                        server::Account::CreateCharacterFailed { reason },
                    ) => {
                        info!("couldn't create character: {reason}");
                        self.creating = false;
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

impl CharacterCreationUI {
    fn initialize<E: GameEngine>(context: &mut Context<E>) -> Self {
        let name_label_text = context.engine.parse_text(WIZARD_16_ID, "Name").unwrap();
        let name_label = Label {
            text: name_label_text,
            position: ((800 / 2), 320),
            color: GRAY_6,
            texture_id: WIZARD_16_ID,
            alignment: Alignment::Center,
        };
        let mut name_input = InputField::new(
            GRAY_6,
            GRAY_1,
            ((800 / 2) - 100, 290),
            (200, 30),
            TAHOMA_BOLD_8_SHADOW_ID,
            INPUT_ID,
            context,
        );
        name_input.focused = true;

        let create_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "Create")
            .unwrap();
        let create_label = Label {
            text: create_text,
            position: (400 - 40 - 10, 196),
            color: GRAY_6,
            texture_id: TAHOMA_BOLD_8_SHADOW_ID,
            alignment: Alignment::Center,
        };
        let create_button = Button {
            position: (400 - 80 - 10, 190),
            size: (80, 20),
            color: GRAY_2,
            texture_id: BUTTON_ID,
            label: Some(create_label),
            ..Default::default()
        };

        Self {
            name_label,
            name_input,

            create_button,
        }
    }
}

impl UI for CharacterCreationUI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let size = screen_size(context.engine);
        let center_x = size.0 / 2;
        let center_y = size.1 / 2;
        self.name_label.position = (center_x, center_y);
        self.name_input.position = (center_x, center_y - 35);
        self.create_button.position = (center_x, center_y - 60);
        if let Some(label) = self.create_button.label.as_mut() {
            label.position = (center_x, center_y - 60);
        }

        self.name_input.update(context);
        self.create_button.update(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        context.engine.draw_image(
            DrawImage {
                position: Position { x: 0, y: 0, z: 0. },
                color: WHITE,
                index: DV_BACKGROUND_ID,
                ..Default::default()
            },
            Target::UI,
        );

        self.name_label.draw(context);
        self.name_input.draw(context);
        self.create_button.draw(context);
    }
}
