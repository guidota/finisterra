#[derive(Clone, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: f32, // can we replace this with an integer?
    pub ui: bool,
}

pub type Color = [u8; 4];

pub mod image {
    use super::{Color, Position};

    pub type Source = [u16; 4];

    #[derive(Clone, Default)]
    pub struct DrawImage {
        pub position: Position,
        pub color: Color,
        pub source: Source,
    }
}

pub mod text {
    use super::{Color, Position};

    pub enum Orientation {
        Center,
        Left,
        Right,
    }

    pub struct DrawText<'s> {
        pub text: &'s str,
        pub position: Position,
        pub color: Color,
        pub orientation: Orientation,
    }
}
