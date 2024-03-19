use std::time::Duration;

use shared::world::Direction;

use super::animation::CharacterAnimation;

#[derive(Debug, Default, Copy, Clone)]
pub struct Animator {
    pub direction: Direction,
    pub animation: CharacterAnimation,
    pub duration: Duration,
    pub time: Duration,
    pub current_frame: usize,
}
