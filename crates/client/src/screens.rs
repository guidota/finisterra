use std::fmt::Display;

use engine::{
    camera::{self, Viewport},
    engine::GameEngine,
};

use crate::game::Context;

use self::{
    account::AccountScreen, character_creation::CharacterCreationScreen, home::HomeScreen,
    world::WorldScreen,
};

pub mod account;
pub mod character_creation;
pub mod demo;
pub mod home;
pub mod world;

pub enum Screen {
    Home(Box<HomeScreen>),
    Demo,
    Account(Box<AccountScreen>),
    CharacterCreation(Box<CharacterCreationScreen>),
    World(Box<WorldScreen>),
}

pub trait GameScreen {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut Context<E>);
    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut Context<E>);
}

impl GameScreen for Screen {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut Context<E>) {
        match self {
            Screen::Home(screen) => screen.update(context),
            Screen::Account(screen) => screen.update(context),
            Screen::CharacterCreation(screen) => screen.update(context),
            Screen::World(screen) => screen.update(context),
            Screen::Demo => todo!(),
        }
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut Context<E>) {
        match self {
            Screen::Home(screen) => screen.draw(context),
            Screen::Account(screen) => screen.draw(context),
            Screen::CharacterCreation(screen) => screen.draw(context),
            Screen::World(screen) => screen.draw(context),
            Screen::Demo => todo!(),
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Screen::Home(_) => f.write_str("Home"),
            Screen::Demo => f.write_str("Demo"),
            Screen::Account(_) => f.write_str("Account"),
            Screen::CharacterCreation(_) => f.write_str("CharacterCreation"),
            Screen::World(_) => f.write_str("World"),
        }
    }
}

pub fn prepare_viewport<E: GameEngine>(context: &mut Context<E>) {
    let window_size = context.engine.get_window_size();
    let height = window_size.height as f32;
    let width = window_size.width as f32;

    let viewport = Viewport {
        x: 0.,
        y: 0.,
        width,
        height,
    };
    context.engine.set_world_camera_viewport(viewport);
    context.engine.set_ui_camera_viewport(viewport);

    if height >= (540. * 2.) && width >= (800. * 2.) {
        context.engine.set_camera_zoom(camera::Zoom::Double);
    } else {
        context.engine.set_camera_zoom(camera::Zoom::None);
    }
}

pub fn screen_size<E: GameEngine>(engine: &mut E) -> (u16, u16) {
    let window_size = engine.get_window_size();
    let zoom = match engine.get_camera_zoom() {
        camera::Zoom::None => 1,
        camera::Zoom::Double => 2,
    };

    (window_size.width / zoom, window_size.height / zoom)
}
