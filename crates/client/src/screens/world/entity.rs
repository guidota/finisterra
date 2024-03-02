use std::time::Duration;

use definitions::{class::Class, race::Race};
use engine::{
    draw::{
        image::DrawImage,
        text::{DrawText, ParsedText},
        Position, Target,
    },
    engine::GameEngine,
};
use lorenzo::{
    animations::Animation,
    character::{animation::CharacterAnimation, animator::Animator, AnimatedCharacter},
};
use protocol::server;
use rand::seq::SliceRandom;

use crate::{
    resources::Resources,
    ui::{
        colors::{BLUE, BLUE_3},
        fonts::TAHOMA_BOLD_8_SHADOW_ID,
    },
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
    pub fn draw<E: GameEngine>(&mut self, engine: &mut E, resources: &Resources) {
        match self {
            Entity::Character(character) => character.draw(engine, resources),
            Entity::Npc(_) => todo!(),
        }
    }
}

pub struct Character {
    pub name: String,
    pub name_text: ParsedText,

    pub position: (u16, u16),

    pub class: Class,
    pub race: Race,

    pub level: u16,
    pub exp: (u16, u16),

    pub mana: (u16, u16),
    pub health: (u16, u16),
    pub energy: (u16, u16),

    animation: AnimatedCharacter,
}

impl Character {
    pub fn from<E: GameEngine>(
        engine: &mut E,
        character: server::Character,
        resources: &Resources,
    ) -> Self {
        let name_text = engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &character.name)
            .expect("can parse");
        let mut animation = Self::random(resources);
        animation.change_animation(CharacterAnimation::Walk);
        Self {
            name: character.name,
            name_text,

            position: (300, 300), // TODO!

            class: Class::Mage,
            race: Race::Human,

            level: 10,
            exp: (0, 100),

            health: (20, 20),
            mana: (100, 100),
            energy: (200, 200),

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

    pub fn draw<E: GameEngine>(&mut self, engine: &mut E, resources: &Resources) {
        let body = self.animation.get_body_frame();

        let x = self.position.0;
        engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.name_text,
                position: Position::new(x, self.position.1 - 14, 0.5),
                color: BLUE,
            },
            Target::World,
        );

        let apoca = engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "Rahma Nanarak O'al")
            .expect("can parse");
        engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &apoca,
                position: Position::new(x, self.position.1 + 56, 0.5),
                color: BLUE_3,
            },
            Target::World,
        );

        let skin = self.animation.get_skin_frame();
        let image = &resources.images[skin.image as usize];

        let x = self.position.0 - body.base.x as u16;
        let y = self.position.1 - body.base.y as u16;

        let color = [255, 255, 255, 255];
        let z = 0.5 + (skin.priority as f32 * 0.0001);

        engine.draw_image(
            DrawImage {
                position: Position::new(x, y, z),
                source: [image.x, image.y, image.width, image.height],
                color,
                index: image.file,
            },
            Target::World,
        );

        if let Some(metadata) = self.animation.get_face_frame() {
            let image = &resources.images[metadata.image as usize];
            engine.draw_image(
                DrawImage {
                    position: Position::new(
                        x + body.head.x as u16 - metadata.offset.x as u16,
                        y + body.head.y as u16 - metadata.offset.y as u16,
                        0.5 + (metadata.priority as f32 * 0.0001),
                    ),
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    index: image.file,
                },
                Target::World,
            );
        }

        if let Some(metadata) = self.animation.get_eyes_frame() {
            let image = &resources.images[metadata.image as usize];
            engine.draw_image(
                DrawImage {
                    position: Position::new(
                        x + body.head.x as u16 - metadata.offset.x as u16,
                        y + body.head.y as u16 - metadata.offset.y as u16,
                        0.5 + (metadata.priority as f32 * 0.0001),
                    ),
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    index: image.file,
                },
                Target::World,
            );
        }

        if let Some(metadata) = self.animation.get_hair_frame() {
            let image = &resources.images[metadata.image as usize];
            engine.draw_image(
                DrawImage {
                    position: Position::new(
                        x + body.head.x as u16 - metadata.offset.x as u16,
                        y + body.head.y as u16 - metadata.offset.y as u16,
                        0.5 + (metadata.priority as f32 * 0.0001),
                    ),
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    index: image.file,
                },
                Target::World,
            );
        }

        if let Some(metadata) = self.animation.get_helmet_frame() {
            let image = &resources.images[metadata.image as usize];
            engine.draw_image(
                DrawImage {
                    position: Position::new(
                        x + body.head.x as u16 - metadata.offset.x as u16,
                        y + body.head.y as u16 - metadata.offset.y as u16,
                        0.5 + (metadata.priority as f32 * 0.0001),
                    ),
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    index: image.file,
                },
                Target::World,
            );
        }

        if let Some(metadata) = self.animation.get_weapon_frame() {
            let image = &resources.images[metadata.image as usize];
            engine.draw_image(
                DrawImage {
                    position: Position::new(
                        x + body.right_hand.x as u16 - metadata.offset.x as u16,
                        y + body.right_hand.y as u16 - metadata.offset.y as u16,
                        0.5 + (metadata.priority as f32 * 0.0001),
                    ),
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    index: image.file,
                },
                Target::World,
            );
        }

        if let Some(metadata) = self.animation.get_shield_frame() {
            let image = &resources.images[metadata.image as usize];
            engine.draw_image(
                DrawImage {
                    position: Position::new(
                        x + body.left_hand.x as u16 - metadata.offset.x as u16,
                        y + body.left_hand.y as u16 - metadata.offset.y as u16,
                        0.5 + (metadata.priority as f32 * 0.0001),
                    ),
                    source: [image.x, image.y, image.width, image.height],
                    color,
                    index: image.file,
                },
                Target::World,
            );
        }
    }
}
