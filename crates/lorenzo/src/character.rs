use std::{ops::AddAssign, time::Duration};

use crate::{animations::ImageFrameMetadata, Offset};

use self::{
    animation::{CharacterAnimation, CharacterAnimations},
    animator::Animator,
    direction::Direction,
};

pub mod animation;
pub mod animator;
pub mod direction;

pub type Armor = CharacterAnimations<ImageFrameMetadata>;
pub type Helmet = CharacterAnimations<ImageFrameMetadata>;
pub type Shield = CharacterAnimations<ImageFrameMetadata>;
pub type Weapon = CharacterAnimations<ImageFrameMetadata>;
pub type Head = CharacterAnimations<ImageFrameMetadata>;
pub type Hair = CharacterAnimations<ImageFrameMetadata>;
pub type Eyes = CharacterAnimations<ImageFrameMetadata>;
pub type Face = CharacterAnimations<ImageFrameMetadata>;
pub type Skin = CharacterAnimations<ImageFrameMetadata>;
pub type Body = CharacterAnimations<BodyFrameMetadata>;

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BodyFrameMetadata {
    pub base: Offset,

    /// Useful to know where to draw face, eyes or hair
    pub head: Offset,

    /// Useful to know where to draw weapons or shields
    pub left_hand: Offset,
    pub right_hand: Offset,

    /// Useful to know when the foot is on the floor (for sounds or effects)
    pub left_foot: Offset,
    pub right_foot: Offset,
}

#[derive(Default, Debug)]
pub struct AnimatedCharacter {
    pub body: Body,
    pub skin: Skin,

    pub eyes: Option<Eyes>,
    pub face: Option<Face>,
    pub hair: Option<Hair>,
    pub armor: Option<Armor>,
    pub shield: Option<Shield>,
    pub helmet: Option<Helmet>,
    pub weapon: Option<Weapon>,

    pub animator: Animator,
}

impl AnimatedCharacter {
    pub fn change_direction(&mut self, direction: Direction) {
        self.animator.direction = direction;
        self.animator.time = Duration::ZERO;
        self.animator.current_frame = 0;
    }

    pub fn change_animation(&mut self, animation: CharacterAnimation) {
        self.animator.animation = animation;
        self.animator.time = Duration::ZERO;
        self.animator.current_frame = 0;
    }

    pub fn update_animation(&mut self, dt: Duration) {
        let animation = &self.animator.animation;
        let direction = &self.animator.direction;
        let frames = self.body[*animation][*direction].frames.len();
        self.animator.time.add_assign(dt);
        let frame_duration = self.animator.duration.as_millis() as usize / frames;
        let current_time = self.animator.time.as_millis() as usize;
        self.animator.current_frame = (current_time / frame_duration) % frames;
    }

    pub fn get_body_frame(&self) -> &BodyFrameMetadata {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;
        &self.body[animation][direction][current_frame]
    }

    pub fn get_skin_frame(&self) -> &ImageFrameMetadata {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        let animation = &self.skin[animation][direction];

        if animation.frames.len() == 1 {
            return &animation[0];
        }
        &animation[current_frame]
    }

    pub fn get_face_frame(&self) -> Option<&ImageFrameMetadata> {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        self.face.as_ref().map(|metadata| {
            let animation = &metadata[animation][direction];
            if animation.frames.len() == 1 {
                return &animation[0];
            }
            &animation[current_frame]
        })
    }

    pub fn get_eyes_frame(&self) -> Option<&ImageFrameMetadata> {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        self.eyes.as_ref().map(|metadata| {
            let animation = &metadata[animation][direction];
            if animation.frames.len() == 1 {
                return &animation[0];
            }
            &animation[current_frame]
        })
    }

    pub fn get_hair_frame(&self) -> Option<&ImageFrameMetadata> {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        self.hair.as_ref().map(|metadata| {
            let animation = &metadata[animation][direction];
            if animation.frames.len() == 1 {
                return &animation[0];
            }
            &animation[current_frame]
        })
    }

    pub fn get_shield_frame(&self) -> Option<&ImageFrameMetadata> {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        self.shield.as_ref().map(|metadata| {
            let animation = &metadata[animation][direction];
            if animation.frames.len() == 1 {
                return &animation[0];
            }
            &animation[current_frame]
        })
    }

    pub fn get_weapon_frame(&self) -> Option<&ImageFrameMetadata> {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        self.weapon.as_ref().map(|metadata| {
            let animation = &metadata[animation][direction];
            if animation.frames.len() == 1 {
                return &animation[0];
            }
            &animation[current_frame]
        })
    }

    pub fn get_helmet_frame(&self) -> Option<&ImageFrameMetadata> {
        let Animator {
            animation,
            direction,
            current_frame,
            ..
        } = self.animator;

        self.helmet.as_ref().map(|metadata| {
            let animation = &metadata[animation][direction];
            if animation.frames.len() == 1 {
                return &animation[0];
            }
            &animation[current_frame]
        })
    }
}
