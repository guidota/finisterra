pub mod animations;
pub mod character;
pub mod image;

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Offset {
    pub x: u8,
    pub y: u8,
}

impl Offset {
    pub const ZERO: Offset = Offset { x: 0, y: 0 };
}
