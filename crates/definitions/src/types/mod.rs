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

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Range {
    pub min: usize,
    pub max: usize,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Offset {
    pub x: usize,
    pub y: usize,
}
