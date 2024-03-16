use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub struct WorldPosition {
    pub map: u16,
    pub x: u16,
    pub y: u16,
}

#[derive(Encode, Decode, PartialEq, Debug, Default, Clone)]
pub enum Direction {
    North,
    East,
    #[default]
    South,
    West,
}
