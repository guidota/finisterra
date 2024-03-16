use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;

use super::{animator::Animator, direction::Direction};

pub const FRAMES: usize = 8;

#[derive(Debug, Default, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CharacterAnimation {
    #[default]
    Idle,
    Walk,
    Attack,
    Defend,
    Die,
}

impl Display for CharacterAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CharacterAnimation::Idle => "idle",
            CharacterAnimation::Walk => "walk",
            CharacterAnimation::Attack => "attack",
            CharacterAnimation::Defend => "defend",
            CharacterAnimation::Die => "die",
        })
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct BodyAnimation<T: Default> {
    #[serde_as(as = "Vec<_>")]
    pub frames: Vec<T>,
}

impl<T: Default + Copy> Default for BodyAnimation<T> {
    fn default() -> Self {
        Self { frames: vec![] }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
#[serde(rename_all = "lowercase")]
pub struct CharacterAnimations<T: Default + Copy> {
    pub idle: Animations<T>,
    pub walk: Animations<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<Animations<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defend: Option<Animations<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub die: Option<Animations<T>>,
}

// pub type Animations<T> = [Animation<T>; DIRECTIONS];
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Animations<T: Default + Copy> {
    pub south: BodyAnimation<T>,
    pub north: BodyAnimation<T>,
    pub east: BodyAnimation<T>,
    pub west: BodyAnimation<T>,
}

impl<T: Default + Copy> Index<CharacterAnimation> for CharacterAnimations<T> {
    type Output = Animations<T>;

    fn index(&self, index: CharacterAnimation) -> &Self::Output {
        match index {
            CharacterAnimation::Idle => &self.idle,
            CharacterAnimation::Walk => &self.walk,
            _ => panic!("Not implemented yet!"),
            // CharacterAnimation::Attack => None,
            // CharacterAnimation::Defend => None,
            // CharacterAnimation::Die => None,
        }
    }
}

impl<T: Default + Copy> IndexMut<CharacterAnimation> for CharacterAnimations<T> {
    fn index_mut(&mut self, index: CharacterAnimation) -> &mut Self::Output {
        match index {
            CharacterAnimation::Idle => &mut self.idle,
            CharacterAnimation::Walk => &mut self.walk,
            _ => panic!("Not implemented yet!"),
            // CharacterAnimation::Attack => None,
            // CharacterAnimation::Defend => None,
            // CharacterAnimation::Die => None,
        }
    }
}

impl<T: Default + Copy> Index<Direction> for Animations<T> {
    type Output = BodyAnimation<T>;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::South => &self.south,
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::West => &self.west,
        }
    }
}

impl<T: Default + Copy> IndexMut<Direction> for Animations<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::South => &mut self.south,
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::West => &mut self.west,
        }
    }
}

impl<T: Default + Copy> Index<usize> for BodyAnimation<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= FRAMES {
            panic!("trying to access an invalid frame");
        }
        &self.frames[index]
    }
}

impl<T: Default + Copy> IndexMut<usize> for BodyAnimation<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= FRAMES {
            panic!("trying to access an invalid frame");
        }
        &mut self.frames[index]
    }
}

impl<T: Default + Copy> CharacterAnimations<T> {
    pub fn get_current(
        &self,
        Animator {
            direction,
            animation,
            current_frame,
            ..
        }: &Animator,
    ) -> &T {
        &self[*animation][*direction][*current_frame]
    }
}
