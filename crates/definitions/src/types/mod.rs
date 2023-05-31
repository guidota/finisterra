pub mod animation;
pub mod body;
pub mod city;
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

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Offset {
    pub x: usize,
    pub y: usize,
}
