use engine::{
    draw::{image::DrawImage, Position, Target},
    engine::GameEngine,
    CursorIcon,
};
use protocol::{
    character::{Class, Gender, Race},
    client, server,
};
use tracing::info;

use crate::{
    game::Context,
    ui::{
        button::{Button, ButtonBuilder},
        colors::*,
        fonts::*,
        input_field::InputField,
        label::Label,
        Widget, UI,
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

    // classes
    pub classes_label: Label,
    pub classes: Vec<Button>,
    pub selected_class: usize,

    // races
    pub races_label: Label,
    pub races: Vec<Button>,
    pub selected_race: usize,

    // gender
    pub genders_label: Label,
    pub genders: Vec<Button>,
    pub selected_gender: usize,

    pub create_button: Button,
}

impl CharacterCreationScreen {
    pub fn new<E: GameEngine>(context: &mut Context<E>) -> Self {
        let ui = CharacterCreationUI::initialize(context);

        context.engine.set_mouse_cursor(CursorIcon::Default);

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
                        class: Class::VALUES[self.ui.selected_class].clone(),
                        race: Race::VALUES[self.ui.selected_race].clone(),
                        gender: Gender::VALUES[self.ui.selected_gender].clone(),
                    },
                ))
        } else {
            for message in messages {
                match message {
                    protocol::server::ServerPacket::Account(
                        server::Account::CreateCharacterOk { character },
                    ) => {
                        let character = entity::Character::from(context, character);

                        context
                            .screen_transition_sender
                            .send(Screen::World(Box::new(WorldScreen::new(
                                context, 0, character,
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
        let name_label = Label::from("Name", WIZARD_16_ID, GRAY_6, context.engine);
        let mut name_input = InputField::new(
            GRAY_6,
            GRAY_1,
            ((800 / 2) - 100, 290),
            (200, 30),
            TAHOMA_BOLD_8_SHADOW_ID,
            context.resources.textures.input,
            context,
        );
        name_input.focused = true;

        let create_label = Label::from("Create", WIZARD_16_ID, GRAY_6, context.engine);
        let create_button = ButtonBuilder::new()
            .color(GRAY_2)
            .texture_id(context.resources.textures.button)
            .label(create_label)
            .build();

        let classes_label = Label::from("Classes", WIZARD_16_ID, GRAY_6, context.engine);
        let mut classes = vec![];
        let default_class = Class::default();
        let mut selected_class = 0;
        for (i, class) in Class::VALUES.iter().enumerate() {
            let class_label = Label::from(
                &class.to_string(),
                TAHOMA_BOLD_8_SHADOW_ID,
                GRAY_6,
                context.engine,
            );
            let mut class_button = ButtonBuilder::new()
                .color(GRAY_2)
                .texture_id(context.resources.textures.button)
                .label(class_label)
                .selected_color(BLUE)
                .build();
            if class == &default_class {
                class_button.select();
                selected_class = i;
            }

            classes.push(class_button);
        }

        let races_label = Label::from("Races", WIZARD_16_ID, GRAY_6, context.engine);
        let mut races = vec![];
        let default_race = Race::default();
        let mut selected_race = 0;
        for (i, race) in Race::VALUES.iter().enumerate() {
            let label = Label::from(
                &race.to_string(),
                TAHOMA_BOLD_8_SHADOW_ID,
                GRAY_6,
                context.engine,
            );
            let mut button = ButtonBuilder::new()
                .color(GRAY_2)
                .texture_id(context.resources.textures.button)
                .label(label)
                .selected_color(BLUE)
                .build();
            if race == &default_race {
                button.select();
                selected_race = i;
            }
            races.push(button);
        }

        let genders_label = Label::from("Genders", WIZARD_16_ID, GRAY_6, context.engine);
        let mut genders = vec![];
        let default_gender = Gender::default();
        let mut selected_gender = 0;
        for (i, gender) in Gender::VALUES.iter().enumerate() {
            let label = Label::from(
                &gender.to_string(),
                TAHOMA_BOLD_8_SHADOW_ID,
                GRAY_6,
                context.engine,
            );
            let mut button = ButtonBuilder::new()
                .color(GRAY_2)
                .texture_id(context.resources.textures.button)
                .label(label)
                .selected_color(BLUE)
                .build();
            if gender == &default_gender {
                button.select();
                selected_gender = i;
            }
            genders.push(button);
        }

        Self {
            name_label,
            name_input,

            create_button,

            classes_label,
            selected_class,
            classes,

            races_label,
            selected_race,
            races,

            genders,
            genders_label,
            selected_gender,
        }
    }
}

impl UI for CharacterCreationUI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let size = screen_size(context.engine);
        let center_x = size.0 / 2;
        let y = 100;
        self.name_label.position = (center_x, y);
        self.name_input.position = (center_x, y - 35);
        self.create_button.position = (center_x, y - 60);
        if let Some(label) = self.create_button.label.as_mut() {
            label.position = (center_x, y - 60);
        }

        let x = 100;
        let mut y = size.1 - (size.1 - self.classes.len() as u16 * 30 - 30) / 2;
        self.classes_label.position = (x, y);
        let mut selection = self.selected_class;
        for (i, button) in self.classes.iter_mut().enumerate() {
            y -= 30;
            button.position = (x, y);
            button.update(context);
            if button.clicked() {
                button.select();
                selection = i;
            }
        }
        if selection != self.selected_class {
            let selected_button = &mut self.classes[self.selected_class];
            selected_button.unselect();
            self.selected_class = selection;
        }

        let x = size.0 - 100;
        let mut y = size.1
            - (size.1 - (self.races.len() as u16 + self.genders.len() as u16) * 30 - 30 * 2) / 2;
        self.races_label.position = (x, y);

        let mut selection = self.selected_race;
        for (i, button) in self.races.iter_mut().enumerate() {
            y -= 30;
            button.position = (x, y);
            button.update(context);
            if button.clicked() {
                button.select();
                selection = i;
            }
        }
        if selection != self.selected_race {
            let selected_button = &mut self.races[self.selected_race];
            selected_button.unselect();
            self.selected_race = selection;
        }

        y -= 30;
        self.genders_label.position = (x, y);
        let mut selection = self.selected_gender;
        for (i, button) in self.genders.iter_mut().enumerate() {
            y -= 30;
            button.position = (x, y);
            button.update(context);
            if button.clicked() {
                button.select();
                selection = i;
            }
        }
        if selection != self.selected_gender {
            let selected_button = &mut self.genders[self.selected_gender];
            selected_button.unselect();
            self.selected_gender = selection;
        }

        self.name_input.update(context);
        self.create_button.update(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        context.engine.draw_image(
            DrawImage {
                position: Position { x: 0, y: 0, z: 0. },
                color: WHITE,
                index: context.resources.textures.dv_background,
                ..Default::default()
            },
            Target::UI,
        );

        self.name_label.draw(context);
        self.name_input.draw(context);
        self.create_button.draw(context);
        self.classes_label.draw(context);
        for button in self.classes.iter_mut() {
            button.draw(context);
        }
        self.races_label.draw(context);
        for button in self.races.iter_mut() {
            button.draw(context);
        }
        self.genders_label.draw(context);
        for button in self.genders.iter_mut() {
            button.draw(context);
        }
    }
}
