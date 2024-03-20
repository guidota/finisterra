use std::{
    ops::{AddAssign, Deref, DerefMut},
    time::Duration,
};

use crate::argentum::{
    animations::ImageFrameMetadata,
    character::{animation::CharacterAnimation, animator::Animator, AnimatedCharacter},
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
use interpolation::{lerp, quad_bez};
use rand::seq::SliceRandom;
use shared::{
    character::{self},
    world::{Direction, WorldPosition},
};

use crate::{
    game::Context,
    resources::Resources,
    ui::{colors::*, fonts::TAHOMA_BOLD_8_SHADOW_ID},
};

use super::{calculate_z, get_direction};

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

    // store 3 last known positions
    just_finished_moving: bool,
    just_started_moving: bool,
    interpolation_time: Duration,
    pub position_buffer: Vec<WorldPosition>,
    pub render_position: (f32, f32),

    name_text: ParsedText,
    pub animation: AnimatedCharacter,

    dialog: Option<Dialog>,
}

struct Dialog {
    text: ParsedText,
    color: Color,
    font_id: FontID,

    total_duration: Duration,
    effect_duration: Duration,
    time: Duration,
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
                position: character.position,
                class: character.class,
                race: character.race,
                look: character.look,
                equipment: character.equipment,
                ..Default::default()
            },
            dialog: None,
            interpolation_time: Duration::ZERO,
            just_finished_moving: false,
            just_started_moving: false,
            position_buffer: vec![],
            render_position: (
                character.position.x as f32 * 32.,
                character.position.y as f32 * 32.,
            ),

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

            interpolation_time: Duration::ZERO,

            position_buffer: vec![],
            render_position: (
                character.position.x as f32 * 32.,
                character.position.y as f32 * 32.,
            ),

            just_started_moving: false,
            just_finished_moving: false,

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
                duration: Duration::from_millis(200),
                ..Default::default()
            },
        }
    }

    pub fn update<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        if self.moving() {
            self.animation.change_animation(CharacterAnimation::Walk);
            self.interpolate(delta);
        } else {
            if self.animation.finished() {
                self.animation.change_animation(CharacterAnimation::Idle);
            }
            self.just_finished_moving = false;
        }

        self.animation.update_animation(delta);
        if let Some(dialog) = self.dialog.as_mut() {
            dialog.update(delta);
            if dialog.finished() {
                self.dialog = None;
            }
        }
    }

    pub fn moving(&self) -> bool {
        !self.position_buffer.is_empty()
    }

    pub fn just_started_moving(&self) -> bool {
        self.just_started_moving
    }

    pub fn just_finished_moving(&self) -> bool {
        self.just_finished_moving
    }

    /// use position buffer to set current position
    pub fn interpolate(&mut self, delta: Duration) {
        if let Some(target) = self.position_buffer.first().cloned() {
            if self.interpolation_time.as_millis() as f32 - delta.as_millis() as f32 <= 0. {
                self.just_finished_moving = true;
                self.position.x = target.x;
                self.position.y = target.y;
                self.position_buffer.remove(0);
                // check tile exit
            } else {
                self.just_finished_moving = false;
                self.just_started_moving = false;
            }
            self.interpolation_time = self
                .interpolation_time
                .checked_sub(delta)
                .unwrap_or(Duration::ZERO);
            let interpolation_progress = 1. - self.interpolation_time.as_millis() as f32 / 200.;
            let x = lerp(
                &(self.position.x as f32),
                &(target.x as f32),
                &interpolation_progress,
            );
            let y = lerp(
                &(self.position.y as f32),
                &(target.y as f32),
                &interpolation_progress,
            );
            self.render_position = (x * 32., y * 32.);

            if self.just_finished_moving && !self.position_buffer.is_empty() {
                self.interpolation_time = Duration::from_millis(200);
                if let Some(direction) = get_direction(&self.position, &self.position_buffer[0]) {
                    self.change_direction(direction);
                }
                self.just_started_moving = true;
            }
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.animation.change_direction(direction);
    }

    pub fn move_to(&mut self, position: WorldPosition) {
        if self.position_buffer.is_empty() {
            if let Some(direction) = get_direction(&self.position, &position) {
                self.change_direction(direction);
            }
            if self.position == position {
                return;
            }
            self.interpolation_time = Duration::from_millis(200);
            self.just_started_moving = true;
        }
        self.position_buffer.push(position);
        if self.position_buffer.len() >= 3 {
            self.position_buffer.remove(0);
        }
    }

    pub fn translate(&mut self, position: WorldPosition) {
        self.position = position;
        self.position_buffer.clear();
        self.just_started_moving = false;
        self.just_finished_moving = false;
    }

    pub fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let body = self.animation.get_body_frame();

        let render_position = self.render_position;

        let x = render_position.0.floor() as u16;
        let y = render_position.1.floor() as u16;
        let z = calculate_z(2, render_position.0 / 32., render_position.1 / 32.);

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

    pub fn render_position(&self) -> (f32, f32) {
        self.render_position
    }
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
