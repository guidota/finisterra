pub mod animations;
pub mod character;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Image {
    pub id: u32,

    /// file number corresponding to the image
    pub file: u32,

    /// source rect, sometimes the image is a portion of a bigger one
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Offset {
    pub x: u8,
    pub y: u8,
}

impl Offset {
    pub const ZERO: Offset = Offset { x: 0, y: 0 };
}
