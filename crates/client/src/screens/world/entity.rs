use std::{
    collections::VecDeque,
    ops::{AddAssign, Deref, DerefMut},
    time::Duration,
};

use argentum::{
    animations::ImageFrameMetadata,
    character::{
        animation::CharacterAnimation, animator::Animator, direction::Direction, AnimatedCharacter,
    },
    Offset,
};
use engine::{
    draw::{
        image::DrawImage,
        text::{DrawText, ParsedText},
        Color, Position, Target,
    },
    engine::{FontID, GameEngine},
};
use interpolation::quad_bez;
use protocol::{
    character::{self},
    movement::Movement,
};
use rand::seq::SliceRandom;

use crate::{
    game::Context,
    resources::Resources,
    ui::{colors::*, fonts::TAHOMA_BOLD_8_SHADOW_ID},
};

use super::calculate_z;

pub enum Entity {
    Character(Character),
    // Npc(AnimatedCharacter),
}

impl Entity {
    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        match self {
            Entity::Character(character) => character.update(engine),
            // Entity::Npc(_) => todo!(),
        }
    }
    pub fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        match self {
            Entity::Character(character) => character.draw(context),
            // Entity::Npc(_) => todo!(),
        }
    }
}

pub struct Character {
    inner: character::Character,

    name_text: ParsedText,
    pub animation: AnimatedCharacter,

    dialog: Option<Dialog>,

    pub movement: Movement,
}

struct Dialog {
    text: ParsedText,
    color: Color,
    font_id: FontID,

    total_duration: Duration,
    effect_duration: Duration,
    time: Duration,
}

impl Dialog {
    pub fn update(&mut self, delta: Duration) {
        self.time.add_assign(delta);
    }

    pub fn draw<E: GameEngine>(&self, engine: &mut E, mut position: Position) {
        let fade = {
            if self.time > self.effect_duration {
                1.
            } else {
                self.time.as_millis() as f32 / self.effect_duration.as_millis() as f32
            }
        };
        let color = shade(self.color, fade);

        let y_offset = quad_bez(&0, &14, &20, &fade) as u16;
        position.y += y_offset;

        engine.draw_text(
            self.font_id,
            DrawText {
                text: &self.text,
                color,
                position,
            },
            Target::World,
        );
    }

    pub fn finished(&self) -> bool {
        self.time.ge(&self.total_duration)
    }
}

impl Deref for Character {
    type Target = character::Character;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Character {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Character {
    pub fn from_preview<E: GameEngine>(
        context: &mut Context<E>,
        character: character::CharacterPreview,
    ) -> Self {
        let name_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &character.name)
            .expect("can parse");
        let mut animation = Self::random(context.resources);
        animation.change_animation(CharacterAnimation::Walk);

        Self {
            name_text,

            inner: character::Character {
                name: character.name,
                description: String::new(),
                level: character.level,
                exp: character.exp,
                gold: character.gold,
                position: character.position.clone(),
                class: character.class,
                race: character.race,
                look: character.look,
                equipment: character.equipment,
                ..Default::default()
            },
            dialog: None,
            movement: Movement {
                input: VecDeque::new(),
                predictions: vec![],
                velocity: 5., // tiles per second
                moving: None,
                position: character.position.clone(),
                moving_position: (0.0, 0.0),
            },

            animation,
        }
    }
    pub fn from<E: GameEngine>(context: &mut Context<E>, character: character::Character) -> Self {
        let name_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &character.name)
            .expect("can parse");
        let mut animation = Self::random(context.resources);
        animation.change_animation(CharacterAnimation::Walk);

        Self {
            name_text,

            movement: Movement {
                input: VecDeque::new(),
                predictions: vec![],
                velocity: 5., // tiles per second
                moving: None,
                position: character.position.clone(),
                moving_position: (0.0, 0.0),
            },
            inner: character,

            dialog: None,

            animation,
        }
    }

    fn random(resources: &Resources) -> AnimatedCharacter {
        let rng = &mut rand::thread_rng();

        let body = resources.bodies.choose(rng).unwrap().clone();
        let skin = resources.skins.choose(rng).unwrap().clone();
        let face = Some(resources.faces.choose(rng).unwrap().clone());
        let eyes = Some(resources.eyes.choose(rng).unwrap().clone());
        let hair = Some(resources.hairs.choose(rng).unwrap().clone());
        let shield = Some(resources.shields.choose(rng).unwrap().clone());
        let helmet = Some(resources.helmets.choose(rng).unwrap().clone());
        let weapon = Some(resources.weapons.choose(rng).unwrap().clone());

        AnimatedCharacter {
            body,
            skin,
            eyes,
            face,
            hair,
            armor: None,
            shield,
            helmet,
            weapon,
            animator: Animator {
                duration: Duration::from_millis(300),
                ..Default::default()
            },
        }
    }

    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        self.animation.update_animation(delta);
        if let Some(dialog) = self.dialog.as_mut() {
            dialog.update(delta);
            if dialog.finished() {
                self.dialog = None;
            }
        }

        self.movement.update(delta);
        if let Some(direction) = self.movement.moving_direction() {
            self.animation.change_direction(to_direction(direction));
            self.animation.change_animation(CharacterAnimation::Walk);
        } else {
            self.animation.change_animation(CharacterAnimation::Idle);
        }
    }

    pub fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let body = self.animation.get_body_frame();
        let render_position = self.movement.world_position();

        let x = (render_position.0 * 32.).floor() as u16;
        let y = (render_position.1 * 32.).floor() as u16;
        let z = calculate_z(2, render_position.0 as f32, render_position.1 as f32);

        // draw shadow
        context.engine.draw_image(
            DrawImage {
                position: Position::new(x - 16, y - 8, z - 0.001),
                color: WHITE,
                index: context.resources.textures.character_shadow,
                ..Default::default()
            },
            Target::World,
        );

        context.engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.name_text,
                position: Position::new(x, y - 14, z),
                color: RED_0,
            },
            Target::World,
        );

        let x = x - body.base.x as u16;
        let y = y - body.base.y as u16;

        let color = [255, 255, 255, 255];

        let mut draw_image = |metadata: Option<&ImageFrameMetadata>, offset: Offset| {
            if let Some(metadata) = metadata {
                let image = &context.resources.images[metadata.image as usize];

                let x = x + offset.x as u16 - metadata.offset.x as u16;
                let y = y + offset.y as u16 - metadata.offset.y as u16;
                let z = z + (metadata.priority as f32 * 0.0001); // TODO! calculate from position in map
                let position = Position::new(x, y, z);

                context.engine.draw_image(
                    DrawImage {
                        position,
                        source: [image.x, image.y, image.width, image.height],
                        color,
                        index: image.file,
                    },
                    Target::World,
                );
            }
        };

        draw_image(Some(self.animation.get_skin_frame()), body.base);
        draw_image(self.animation.get_face_frame(), body.head);
        draw_image(self.animation.get_eyes_frame(), body.head);
        draw_image(self.animation.get_hair_frame(), body.head);
        draw_image(self.animation.get_helmet_frame(), body.head);
        draw_image(self.animation.get_weapon_frame(), body.right_hand);
        draw_image(self.animation.get_shield_frame(), body.left_hand);

        if let Some(dialog) = self.dialog.as_mut() {
            let x = x + body.head.x as u16;
            let body_frame_metadata = self.animation.body.idle.south.frames[0];
            let head_y = body_frame_metadata.head.y as u16;
            let y = y + head_y;
            dialog.draw(context.engine, Position::new(x, y, z));
        }
    }

    pub fn add_dialog<E: GameEngine>(
        &mut self,
        engine: &mut E,
        text: &str,
        font_id: FontID,
        color: Color,
    ) {
        let text = engine.parse_text(font_id, text).expect("can parse text");
        self.dialog = Some(Dialog {
            text,
            color,
            font_id,
            total_duration: Duration::from_secs(15),
            effect_duration: Duration::from_millis(150),
            time: Duration::ZERO,
        });
    }
}

fn to_direction(direction: protocol::world::Direction) -> Direction {
    match direction {
        protocol::world::Direction::North => Direction::North,
        protocol::world::Direction::East => Direction::East,
        protocol::world::Direction::South => Direction::South,
        protocol::world::Direction::West => Direction::West,
    }
}
