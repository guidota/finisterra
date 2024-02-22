use engine::engine::GameEngine;

use self::{fonts::Fonts, textures::Textures};

pub mod button;
pub mod colors;
pub mod fonts;
pub mod image;
pub mod input_field;
pub mod label;
pub mod textures;

#[derive(Default)]
pub enum Alignment {
    Left,
    #[default]
    Center,
    Right,
}

pub trait Widget {
    fn update<E: GameEngine>(&mut self, engine: &mut E);
    fn draw<E: GameEngine>(&mut self, engine: &mut E);
}

pub trait UI {
    fn update<E: GameEngine>(&mut self, engine: &mut E);
    fn draw<E: GameEngine>(&mut self, engine: &mut E);
}

pub fn load<E: GameEngine>(engine: &mut E) {
    Textures::load(engine);
    Fonts::load(engine);
}
