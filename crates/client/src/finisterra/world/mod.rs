use definitions::map::Map;
use engine::engine::GameEngine;

use self::entity::Entity;

mod entity;
mod input;
mod settings;
mod ui;
mod z_ordering;

pub struct World {
    pub current_map: Map,
    pub position: (f32, f32),
    pub entities: Vec<Entity>,

    pub window_size: (usize, usize),
    pub render_size: (usize, usize),

    pub map_layer_textures: [RenderTarget; 4],

    pub draw_entities: bool,
    pub draw_names: bool,
    pub draw_map: bool,
}

#[derive(Debug)]
pub enum RenderTarget {
    Uninitialized,
    Dirty { texture_id: u64 },
    Ready { texture_id: u64 },
}

#[derive(Debug)]
pub enum TextureState {
    Dirty,
    Ready,
}

impl World {
    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {}
    pub fn render<E: GameEngine>(&mut self, engine: &mut E) {}
}
