use bincode::{Decode, Encode};

use crate::world::{Direction, WorldPosition};

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
pub struct MoveRequest {
    pub id: u8,
    pub direction: Direction,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
pub struct MoveResponse {
    pub request_id: u8,
    pub position: WorldPosition,
}
