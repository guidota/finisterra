pub mod animations;
pub mod atlas;
pub mod body;
pub mod character;
pub mod city;
pub mod class;
pub mod fx;
pub mod gear;
pub mod head;
pub mod heading;
pub mod image;
pub mod map;
pub mod npc;
pub mod object;
pub mod race;
pub mod shield;
pub mod spell;
pub mod weapon;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Range {
    pub min: usize,
    pub max: usize,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Offset {
    pub x: u8,
    pub y: u8,
}

impl Offset {
    pub const ZERO: Offset = Offset { x: 0, y: 0 };
}
