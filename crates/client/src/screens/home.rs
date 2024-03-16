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
    ui::UI,
    ui::{
        button::{Button, ButtonBuilder},
        colors::GRAY_6,
        input_field::InputField,
        label::Label,
        Widget,
    },
    ui::{
        fonts::{TAHOMA_BOLD_8_SHADOW_ID, WIZARD_16_ID},
        Alignment,
    },
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
                    name: user.to_string(),
                    password: password.to_string(),
                }))
        } else if self.ui.register_button.clicked() {
            self.connecting = true;
            context
                .connection
                .send(ClientPacket::Account(client::Account::CreateAccount {
                    name: user.to_string(),
                    email: user.to_string(),
                    password: password.to_string(),
                    pin: 0,
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

        let user_label = Label::from("User", WIZARD_16_ID, GRAY_6, context.engine);
        let mut user_input = InputField::new(
            GRAY_6,
            GRAY_1,
            (center_x, 290),
            (200, 30),
            TAHOMA_BOLD_8_SHADOW_ID,
            context.resources.textures.input,
            context,
        );
        user_input.focused = true;

        let password_label = Label::from("Password", WIZARD_16_ID, GRAY_6, context.engine);
        let mut password_input = InputField::new(
            GRAY_6,
            GRAY_1,
            (center_x, 230),
            (200, 30),
            TAHOMA_BOLD_8_SHADOW_ID,
            context.resources.textures.input,
            context,
        );
        password_input.obfuscate = true;

        let login_label = Label::from("Log in", TAHOMA_BOLD_8_SHADOW_ID, GRAY_6, context.engine);
        let login_button = ButtonBuilder::new()
            .color(BLUE)
            .label(login_label)
            .texture_id(context.resources.textures.button)
            .alignment(Alignment::Left)
            .z(0.9)
            .build();

        let register_label =
            Label::from("Register", TAHOMA_BOLD_8_SHADOW_ID, GRAY_6, context.engine);
        let register_button = ButtonBuilder::new()
            .color(GRAY_2)
            .label(register_label)
            .texture_id(context.resources.textures.button)
            .alignment(Alignment::Right)
            .z(0.9)
            .build();

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
        let center_y = size.1 / 2;

        self.user_label.position = (center_x, center_y + 50);
        self.user_input.position = (center_x, center_y + 20);
        self.password_label.position = (center_x, center_y + 10);
        self.password_input.position = (center_x, center_y - 20);
        self.login_button.position = (center_x + 10, center_y - 50);
        self.register_button.position = (center_x - 10, center_y - 50);

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
                index: context.resources.textures.dv_background,
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
