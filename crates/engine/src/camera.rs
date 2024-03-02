#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy)]
pub enum Zoom {
    None,
    Double,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
