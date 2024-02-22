use crate::engine::TextureID;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: f32, // can we replace this with an integer?
}

impl Position {
    pub fn new(x: u16, y: u16, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub type Color = [u8; 4];

pub mod image {
    use super::{Color, Position};

    pub type Source = [u16; 4];

    #[repr(C)]
    #[derive(Debug, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
    pub struct DrawImage {
        pub position: Position,
        pub color: Color,
        pub source: Source,
        pub index: u32,
    }

    impl Default for DrawImage {
        fn default() -> Self {
            Self {
                position: Default::default(),
                color: [255, 255, 255, 255],
                source: [0, 0, 0, 0],
                index: Default::default(),
            }
        }
    }
}

pub mod text {
    use super::{Color, Position};

    pub enum Orientation {
        Center,
        Left,
        Right,
    }

    #[derive(Debug)]
    pub struct ParsedText {
        pub chars: Vec<bmfont::CharPosition>,
        pub total_width: u16,
    }

    pub struct DrawText<'s> {
        pub text: &'s ParsedText,
        pub position: Position,
        pub color: Color,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy)]
pub enum Target {
    World,
    UI,
    Texture { id: TextureID },
}
