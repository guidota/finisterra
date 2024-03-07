use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use engine::{
    draw::{
        image::DrawImage,
        text::{DrawText, ParsedText},
        Position, Target,
    },
    engine::GameEngine,
};
use lorenzo::{
    animations::ImageFrameMetadata,
    character::{animation::CharacterAnimation, animator::Animator, AnimatedCharacter},
    Offset,
};
use protocol::character::{self};
use rand::seq::SliceRandom;

use crate::{
    game::Context,
    resources::Resources,
    ui::{colors::BLUE, fonts::TAHOMA_BOLD_8_SHADOW_ID},
};

pub enum Entity {
    Character(Character),
    Npc(AnimatedCharacter),
}

impl Entity {
    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        match self {
            Entity::Character(character) => character.update(engine),
            Entity::Npc(_) => todo!(),
        }
    }
    pub fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        match self {
            Entity::Character(character) => character.draw(context),
            Entity::Npc(_) => todo!(),
        }
    }
}

pub struct Character {
    name_text: ParsedText,
    inner: character::Character,

    // visual aspect
    animation: AnimatedCharacter,
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
    pub fn from<E: GameEngine>(context: &mut Context<E>, character: character::Character) -> Self {
        let name_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &character.name)
            .expect("can parse");
        let mut animation = Self::random(context.resources);
        animation.change_animation(CharacterAnimation::Walk);
        Self {
            name_text,

            inner: character,

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
                duration: Duration::from_millis(500),
                ..Default::default()
            },
        }
    }

    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        self.animation.update_animation(delta);
    }

    pub fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let body = self.animation.get_body_frame();

        let x = self.inner.position.x;
        context.engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.name_text,
                position: Position::new(x, self.inner.position.y - 14, 0.5),
                color: BLUE,
            },
            Target::World,
        );

        let x = self.inner.position.x - body.base.x as u16;
        let y = self.inner.position.y - body.base.y as u16;

        let color = [255, 255, 255, 255];

        let mut draw_image = |metadata: Option<&ImageFrameMetadata>, offset: Offset| {
            if let Some(metadata) = metadata {
                let image = &context.resources.images[metadata.image as usize];
                context.engine.draw_image(
                    DrawImage {
                        position: Position::new(
                            x + offset.x as u16 - metadata.offset.x as u16,
                            y + offset.y as u16 - metadata.offset.y as u16,
                            0.5 + (metadata.priority as f32 * 0.0001),
                        ),
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
    }
}
