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
        Color, Dimensions, Position, Target,
    },
    engine::{FontID, GameEngine, TextureID},
};
use interpolation::quad_bez;
use rand::{seq::SliceRandom, Rng};
use shared::{
    character::{self},
    world::{Direction, WorldPosition},
};

use crate::{
    game::Context,
    resources::Resources,
    ui::{colors::*, fonts::TAHOMA_BOLD_8_SHADOW_ID},
};

use super::{depth::Z, TILE_SIZE_F};

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
    pub fn draw<E: GameEngine>(&mut self, engine: &mut E, resources: &Resources) {
        match self {
            Entity::Character(character) => {
                if character.is_invisible() {
                    character.draw_to_texture(engine, resources);
                }
                character.draw(engine, resources)
            } // Entity::Npc(_) => todo!(),
        }
    }
}

pub struct Character {
    pub inner: character::Character,

    pub just_finished_moving: bool,
    pub just_started_moving: bool,
    pub interpolation_time: Duration,
    pub position_buffer: Vec<WorldPosition>,
    pub render_position: (f32, f32),

    name_text: ParsedText,
    pub animation: AnimatedCharacter,

    dialog: Option<Dialog>,
    clan_text: Option<ParsedText>,
    invisible: Option<Invisibility>,
    texture: Option<(TextureID, TextureState)>,
}

struct Dialog {
    text: ParsedText,
    color: Color,
    font_id: FontID,

    total_duration: Duration,
    effect_duration: Duration,
    time: Duration,
}

struct Invisibility {
    duration: Duration,
    time: Duration,
    name_color: Color,
}

#[derive(Debug)]
enum TextureState {
    Dirty,
    JustDraw,
    Ready,
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
        let clan_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "<Finisterra>")
            .expect("can parse");
        let mut animation = Self::random(context.resources);
        animation.change_animation(CharacterAnimation::Idle);

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

            clan_text: Some(clan_text),
            dialog: None,
            interpolation_time: Duration::ZERO,
            just_finished_moving: false,
            just_started_moving: false,
            position_buffer: vec![],
            render_position: (
                character.position.x as f32 * TILE_SIZE_F,
                (character.position.y as f32 * TILE_SIZE_F) - TILE_SIZE_F / 2.,
            ),

            invisible: None,
            texture: None,

            animation,
        }
    }
    pub fn from<E: GameEngine>(context: &mut Context<E>, character: character::Character) -> Self {
        let name_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &character.name)
            .expect("can parse");
        let clan_text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "<Finisterra>")
            .expect("can parse");

        let mut animation = Self::random(context.resources);
        animation.change_animation(CharacterAnimation::Walk);

        Self {
            name_text,
            clan_text: Some(clan_text),

            interpolation_time: Duration::ZERO,

            position_buffer: vec![],
            render_position: (
                character.position.x as f32 * TILE_SIZE_F,
                character.position.y as f32 * TILE_SIZE_F,
            ),

            just_started_moving: false,
            just_finished_moving: false,

            inner: character,

            dialog: None,
            invisible: None,
            texture: None,

            animation,
        }
    }

    pub fn random(resources: &Resources) -> AnimatedCharacter {
        let rng = &mut rand::thread_rng();

        let (body, skins) = resources.bodies.choose(rng).unwrap().clone();
        let skin = skins.choose(rng).unwrap().clone();
        let face = Some(resources.faces.choose(rng).unwrap().clone());
        let eyes = Some(resources.eyes.choose(rng).unwrap().clone());
        let hair = Some(resources.hairs.choose(rng).unwrap().clone());
        let shield = if rng.gen_bool(0.5) {
            resources.shields.choose(rng).cloned()
        } else {
            None
        };
        let helmet = if rng.gen_bool(0.5) {
            resources.helmets.choose(rng).cloned()
        } else {
            None
        };
        let weapon = if rng.gen_bool(0.5) {
            resources.weapons.choose(rng).cloned()
        } else {
            None
        };
        let clothing = if rng.gen_bool(0.5) {
            resources.clothing.choose(rng).cloned()
        } else {
            None
        };

        AnimatedCharacter {
            body,
            skin,
            eyes,
            face,
            hair,
            clothing,
            shield,
            helmet,
            weapon,
            animator: Animator {
                duration: Duration::from_millis(400),
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

        if let Some(invisibility) = self.invisible.as_mut() {
            invisibility.update(engine.get_delta());

            if invisibility.finished() {
                self.invisible = None;
            }
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

    pub fn change_direction(&mut self, direction: Direction) {
        self.animation.change_direction(direction);
    }

    pub fn move_to(&mut self, position: WorldPosition) {
        if self.position_buffer.is_empty() {
            if let Some(direction) = self.position.get_direction(&position) {
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

    pub fn draw<E: GameEngine>(&self, engine: &mut E, resources: &Resources) {
        let body = self.animation.get_body_frame();
        let render_position = self.render_position;

        let x = render_position.0 as u16;
        let y = render_position.1 as u16;
        let z = Z[2][(render_position.0 / TILE_SIZE_F) as usize]
            [(render_position.1 / TILE_SIZE_F) as usize];

        let name_color = self
            .invisible
            .as_ref()
            .map(|_| transparent(RED_0, 120))
            .unwrap_or(transparent(RED_0, 244));

        engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.name_text,
                position: Position::new(x + 17, y, z + 0.001),
                color: name_color,
            },
            Target::World,
        );

        if let Some(clan_text) = self.clan_text.as_ref() {
            let name_color = self
                .invisible
                .as_ref()
                .map(|invisibility| invisibility.name_color)
                .unwrap_or(transparent(RED_0, 244));
            engine.draw_text(
                TAHOMA_BOLD_8_SHADOW_ID,
                DrawText {
                    text: clan_text,
                    position: Position::new(x + 17, y - 13, z + 0.001),
                    color: name_color,
                },
                Target::World,
            );
        }

        if let Some(invisibility) = self.invisible.as_ref() {
            if let Some((texture, state)) = self.texture.as_ref() {
                if matches!(state, TextureState::Ready) {
                    let transparency = invisibility.transparency();
                    let animation = self.animation.animator.animation;
                    let direction = self.animation.animator.direction;
                    let frame = self.animation.animator.current_frame;

                    let mut offset_y = match direction {
                        Direction::South => 64 * 3,
                        Direction::North => 64 * 2,
                        Direction::East => 64,
                        Direction::West => 0,
                    };
                    if let CharacterAnimation::Idle = animation {
                        offset_y += 4 * 64
                    };

                    let offset_x = frame * 64;
                    engine.draw_image(
                        DrawImage {
                            position: Position::new(x + 16 - 32, y, z),
                            color: transparent(GRAY_6, transparency),
                            index: *texture,
                            source: [offset_x as u16, offset_y as u16, 64, 64],
                        },
                        Target::World,
                    );
                }
            }
        } else {
            self.draw_char(
                resources,
                engine,
                &self.animation,
                (x + 16, y + 16, z),
                Target::World,
            );
        }

        let x = x + 16 - body.base.x as u16;
        let y = y + 16 - body.base.y as u16;
        if let Some(dialog) = self.dialog.as_ref() {
            let body_frame_metadata = self.animation.body.idle.south.frames[0];
            let head_y = body_frame_metadata.head.y as u16;
            let head_x = body_frame_metadata.head.x as u16;
            let x = x + head_x + 1;
            let y = y + head_y;
            dialog.draw(engine, Position::new(x, y, 0.99));
        }
    }

    fn draw_char<E: GameEngine>(
        &self,
        resources: &Resources,
        engine: &mut E,
        animation: &AnimatedCharacter,
        (x, y, z): (u16, u16, f32),
        target: Target,
    ) {
        let color = [255, 255, 255, 255];
        let body = animation.get_body_frame();
        let x = x - body.base.x as u16;
        let y = y - body.base.y as u16;

        let mut draw_image = |metadata: Option<&ImageFrameMetadata>, offset: Offset| {
            if let Some(metadata) = metadata {
                let image = &resources.images[metadata.image as usize];

                let x = x + offset.x as u16 - metadata.offset.x as u16;
                let y = y + offset.y as u16 - metadata.offset.y as u16;
                let z = z + (metadata.priority as f32 * 0.0001);
                let position = Position::new(x, y, z);

                engine.draw_image(
                    DrawImage {
                        position,
                        source: [image.x, image.y, image.width, image.height],
                        color,
                        index: image.file,
                    },
                    target,
                );
            }
        };

        draw_image(Some(animation.get_skin_frame()), body.base);
        draw_image(animation.get_clothing_frame(), body.base);
        draw_image(animation.get_face_frame(), body.head);
        draw_image(animation.get_eyes_frame(), body.head);
        draw_image(animation.get_hair_frame(), body.head);
        draw_image(animation.get_helmet_frame(), body.head);
        draw_image(animation.get_weapon_frame(), body.right_hand);
        draw_image(animation.get_shield_frame(), body.left_hand);
    }

    fn draw_to_texture<E: GameEngine>(&mut self, engine: &mut E, resources: &Resources) {
        if let Some((texture, state)) = self.texture.as_ref() {
            if matches!(state, TextureState::Ready) {
                return;
            }
            if matches!(state, TextureState::JustDraw) {
                self.texture = Some((*texture, TextureState::Ready));
                return;
            }
        }
        tracing::info!("drawing to texture");

        let texture_id = self.texture.as_ref().unwrap().0;
        let mut animation = self.animation.clone();

        let mut y = 0;
        for char_animation in [CharacterAnimation::Idle, CharacterAnimation::Walk] {
            animation.change_animation(char_animation);
            for direction in [
                Direction::South,
                Direction::North,
                Direction::East,
                Direction::West,
            ] {
                animation.change_direction(direction);
                let mut x = 0;
                for i in 0..8 {
                    animation.animator.current_frame = i;
                    self.draw_char(
                        resources,
                        engine,
                        &animation,
                        (x + 32, y + 16, 0.5),
                        Target::Texture { id: texture_id },
                    );
                    x += 64;
                }
                y += 64;
            }
        }

        self.texture = Some((texture_id, TextureState::JustDraw));
    }

    pub fn is_invisible(&self) -> bool {
        self.invisible.is_some()
    }

    pub fn remove_invisible(&mut self) {
        self.invisible = None;
    }

    pub fn set_invisible<E: GameEngine>(
        &mut self,
        engine: &mut E,
        duration: Duration,
        color: Color,
    ) {
        if self.invisible.is_some() {
            return;
        }

        if self.texture.is_none() {
            // todo: use body to determine dimensions
            let dimensions = Dimensions {
                width: 64 * 8,
                height: 64 * 8,
            };
            self.texture = Some((engine.create_texture(dimensions), TextureState::Dirty));
        }

        self.invisible = Some(Invisibility {
            duration,
            time: Duration::ZERO,
            name_color: color,
        });
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

impl Invisibility {
    pub fn update(&mut self, delta: Duration) {
        self.time.add_assign(delta);
    }

    pub fn finished(&self) -> bool {
        self.time > self.duration
    }

    fn transparency(&self) -> u8 {
        let progress = self.time.as_millis() as f32 / self.duration.as_millis() as f32
            * 4.
            * std::f32::consts::PI;
        let progress = progress.sin() - 0.5;

        if progress > 0. && progress < 1. {
            (progress * 256.) as u8
        } else {
            0
        }
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
