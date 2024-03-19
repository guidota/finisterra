use engine::engine::GameEngine;

use crate::game::Context;

pub mod bar;
pub mod button;
pub mod colors;
pub mod fonts;
pub mod input_field;
pub mod label;
pub mod texture;
pub mod textures;

#[derive(Default, Debug)]
pub enum Alignment {
    Left,
    #[default]
    Center,
    Right,
}

pub trait Widget {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>);
    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>);
}

pub trait UI {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>);
    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>);
}
