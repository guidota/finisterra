use engine::{
    draw::{image::DrawImage, Position, Target},
    engine::GameEngine,
    input::keyboard::KeyCode,
};
use protocol::{
    client::{self, ClientPacket},
    server::{self, ServerPacket},
};

use crate::{
    game::Context,
    ui::colors::*,
    ui::{button::Button, colors::GRAY_6, input_field::InputField, label::Label, Widget},
    ui::{
        fonts::{TAHOMA_BOLD_8_SHADOW_ID, WIZARD_16_ID},
        Alignment,
    },
    ui::{textures::*, UI},
};

use super::{account::AccountScreen, prepare_viewport, screen_size, GameScreen, Screen};

pub struct HomeScreen {
    ui: HomeUI,

    connecting: bool,
}

pub struct HomeUI {
    // user
    user_label: Label,
    pub user_input: InputField,

    // password
    password_label: Label,
    pub password_input: InputField,

    pub login_button: Button,
    pub register_button: Button,
}

impl HomeScreen {
    pub fn new<E: GameEngine>(context: &mut Context<E>) -> Self {
        let ui = HomeUI::initialize(context);
        Self {
            ui,
            connecting: false,
        }
    }
}

impl GameScreen for HomeScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        prepare_viewport(context);

        self.ui.update(context);

        let user = self.ui.user_input.text();
        let password = self.ui.password_input.text();
        let messages = context.connection.read();

        if self.connecting {
            if !context.connection.is_connected() {
                self.connecting = false;
            }

            for message in messages {
                match message {
                    ServerPacket::Account(server::Account::LoginOk { characters }) => {
                        // move to account screen
                        context
                            .screen_transition_sender
                            .send(Screen::Account(Box::new(AccountScreen::new(
                                context, characters,
                            ))))
                            .expect("poisoned");
                    }

                    ServerPacket::Account(server::Account::Created { .. }) => {
                        context
                            .screen_transition_sender
                            .send(Screen::Account(Box::new(AccountScreen::new(
                                context,
                                vec![],
                            ))))
                            .expect("poisoned");
                    }
                    ServerPacket::Account(server::Account::LoginFailed) => self.connecting = false,
                    ServerPacket::Account(server::Account::CreateFailed { .. }) => {
                        self.connecting = false
                    }
                    _ => {}
                }
            }
        } else if self.ui.login_button.clicked() || context.engine.key_released(KeyCode::Enter) {
            self.connecting = true;
            context
                .connection
                .send(ClientPacket::Account(client::Account::LoginAccount {
                    mail: user.to_string(),
                    password: password.to_string(),
                }))
        } else if self.ui.register_button.clicked() {
            self.connecting = true;
            context
                .connection
                .send(ClientPacket::Account(client::Account::CreateAccount {
                    mail: user.to_string(),
                    password: password.to_string(),
                }))
        }
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.ui.draw(context);
    }
}

impl HomeUI {
    pub fn initialize<E: GameEngine>(context: &mut Context<E>) -> Self {
        let size = context.engine.get_window_size();
        let center_x = size.width / 2;

        let user_label_text = context.engine.parse_text(WIZARD_16_ID, "User").unwrap();
        let user_label = Label {
            text: user_label_text,
            position: (center_x, 320),
            color: GRAY_6,
            texture_id: WIZARD_16_ID,
            alignment: Alignment::Center,
        };
        let mut user_input = InputField::new(
            GRAY_6,
            GRAY_1,
            (center_x, 290),
            (200, 30),
            TAHOMA_BOLD_8_SHADOW_ID,
            INPUT_ID,
            context,
        );
        user_input.focused = true;

        let password_label_text = context.engine.parse_text(WIZARD_16_ID, "Password").unwrap();
        let password_label = Label {
            text: password_label_text,
            position: (center_x, 260),
            color: GRAY_6,
            texture_id: WIZARD_16_ID,
            alignment: Alignment::Center,
        };
        let mut password_input = InputField::new(
            GRAY_6,
            GRAY_1,
            (center_x, 230),
            (200, 30),
            TAHOMA_BOLD_8_SHADOW_ID,
            INPUT_ID,
            context,
        );
        password_input.obfuscate = true;

        let login_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "Log in")
            .unwrap();
        let login_label = Label {
            text: login_text,
            position: (0, 10),
            color: GRAY_6,
            texture_id: TAHOMA_BOLD_8_SHADOW_ID,
            alignment: Alignment::Center,
        };
        let login_button = Button {
            position: (center_x + 10, 190),
            size: (80, 20),
            color: BLUE,
            texture_id: BUTTON_ID,
            label: Some(login_label),
            alignment: Alignment::Left,

            ..Default::default()
        };

        let register_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "Register")
            .unwrap();
        let register_label = Label {
            text: register_text,
            position: (0, 10),
            color: GRAY_6,
            texture_id: TAHOMA_BOLD_8_SHADOW_ID,
            alignment: Alignment::Center,
        };
        let register_button = Button {
            position: (center_x - 10, 190),
            size: (80, 20),
            color: GRAY_2,
            texture_id: BUTTON_ID,
            label: Some(register_label),
            alignment: Alignment::Right,
            ..Default::default()
        };

        Self {
            user_input,
            user_label,

            password_input,
            password_label,

            login_button,
            register_button,
        }
    }
}

impl UI for HomeUI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if context.engine.key_pressed(KeyCode::Tab) {
            self.user_input.focused = !self.user_input.focused;
            self.password_input.focused = !self.password_input.focused;
        } else if context.engine.key_pressed(KeyCode::Enter) {
            // send connect
        }

        let size = screen_size(context.engine);
        let center_x = size.0 / 2;

        self.user_label.position = (center_x, self.user_label.position.1);
        self.user_input.position = (center_x, self.user_input.position.1);
        self.password_label.position = (center_x, self.password_label.position.1);
        self.password_input.position = (center_x, self.password_input.position.1);
        self.login_button.position = (center_x + 10, self.login_button.position.1);
        self.register_button.position = (center_x - 10, self.register_button.position.1);

        self.user_input.update(context);
        self.password_input.update(context);
        self.login_button.update(context);
        self.register_button.update(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        context.engine.draw_image(
            DrawImage {
                position: Position { x: 0, y: 0, z: 0. },
                color: [255, 255, 255, 255],
                index: DV_BACKGROUND_ID,
                source: [0, 0, 0, 0],
            },
            Target::UI,
        );

        self.user_label.draw(context);
        self.user_input.draw(context);
        self.password_label.draw(context);
        self.password_input.draw(context);
        self.login_button.draw(context);
        self.register_button.draw(context);
    }
}
