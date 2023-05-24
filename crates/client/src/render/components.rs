use bevy_ecs::prelude::Component;
use roma::Buffer;

#[derive(Component)]
pub struct Sprite {
    pub id: String,
}

#[derive(Component)]
pub struct SpriteBuffers {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

#[derive(Component)]
pub struct Animation {
    pub id: String,
    // timer: Timer,
}

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
