use std::{ops::Add, rc::Rc, time::Duration};

use definitions::{
    animation::Animation, body::Body, client::ClientResources, head::Head, heading::Heading,
    image::Image, shield::Shield, weapon::Weapon,
};
use rand::{seq::IteratorRandom, Rng};
use rnglib::{Language, RNG};
use roma::{parse_text, ParsedText, SmolStr};

use crate::{HALF_TILE, TILE_SIZE};

#[derive(Debug)]
pub enum Movement {
    Idle,
    Walking {
        animation_time: Duration,
        current_time: Duration,
    },
}

#[derive(Debug)]
pub struct State {
    pub movement: Movement,
    pub direction: Heading,
}

type Animated<T> = (T, ([Rc<Animation>; 4], usize));

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub body: Option<Animated<Body>>,
    pub head: Option<(Head, [Rc<Image>; 4])>,
    pub weapon: Option<Animated<Weapon>>,
    pub shield: Option<Animated<Shield>>,
    pub state: State,
    pub name: (SmolStr, ParsedText),
    pub position: [f32; 2],
    pub world_position: [f32; 2],
}

lazy_static::lazy_static! {
    static ref NAME_GENERATOR: RNG = RNG::try_from(&Language::Fantasy).unwrap();
}

impl Entity {
    pub fn random(id: usize, resources: &ClientResources) -> Self {
        let mut rng = rand::thread_rng();
        let direction = match rng.gen_range(0..4) {
            0 => Heading::South,
            1 => Heading::North,
            2 => Heading::West,
            3 => Heading::East,
            _ => Heading::South,
        };

        let get_animations = |animations: [usize; 4]| {
            let animations = [
                resources.animations[animations[0]].clone()?,
                resources.animations[animations[1]].clone()?,
                resources.animations[animations[2]].clone()?,
                resources.animations[animations[3]].clone()?,
            ];

            Some(animations)
        };

        let (random_body, body_animations) = loop {
            let (_, body) = resources.bodies.iter().choose(&mut rng).unwrap();
            let Some(animations) = get_animations(body.animations) else {
                continue;
            };
            break (body.clone(), animations);
        };
        let (random_head, head_images) = loop {
            let (_, head) = resources.heads.iter().choose(&mut rng).unwrap();
            if head.images[..4].iter().any(|image| image == &0) {
                continue;
            }
            let images = [
                resources.images[head.images[0]].clone().unwrap(),
                resources.images[head.images[1]].clone().unwrap(),
                resources.images[head.images[2]].clone().unwrap(),
                resources.images[head.images[3]].clone().unwrap(),
            ];
            break (head.clone(), images);
        };
        // let (random_weapon, weapon_animations) = loop {
        //     let (_, weapon) = resources.weapons.iter().choose(&mut rng).unwrap();
        //     let Some(animations) = get_animations(weapon.animations) else {
        //         continue;
        //     };
        //     break (weapon.clone(), animations);
        // };
        //
        // let (random_shield, shield_animations) = loop {
        //     let (_, shield) = resources.shields.iter().choose(&mut rng).unwrap();
        //     let Some(animations) = get_animations(shield.animations) else {
        //         continue;
        //     };
        //     break (shield.clone(), animations);
        // };

        let x = rng.gen_range(0..100) as f32;
        let y = rng.gen_range(0..100) as f32;

        let name = SmolStr::new(NAME_GENERATOR.generate_name());
        let parsed_text = parse_text(&name);
        Self {
            id,
            body: Some((random_body, (body_animations, 0))),
            head: Some((random_head, head_images)),
            // weapon: Some((random_weapon, (weapon_animations, 0))),
            // shield: Some((random_shield, (shield_animations, 0))),
            weapon: None,
            shield: None,
            position: [x, y],
            world_position: [
                x * TILE_SIZE as f32 + HALF_TILE as f32,
                y * TILE_SIZE as f32,
            ],
            name: (name, parsed_text),
            state: State {
                movement: if rng.gen_bool(0.5) {
                    Movement::Walking {
                        animation_time: Duration::from_millis(500),
                        current_time: Duration::from_millis(0),
                    }
                } else {
                    Movement::Idle
                },
                direction,
            },
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if let Movement::Walking {
            animation_time,
            ref mut current_time,
        } = &mut self.state.movement
        {
            *current_time = current_time.add(delta);
            if current_time > animation_time {
                *current_time = Duration::from_millis(0);
            }

            let direction = self.state.direction;
            update_animation(animation_time, current_time, direction, self.body.as_mut());
            update_animation(
                animation_time,
                current_time,
                direction,
                self.shield.as_mut(),
            );
            update_animation(
                animation_time,
                current_time,
                direction,
                self.weapon.as_mut(),
            );
        }
    }
}

fn update_animation<T>(
    animation_time: &Duration,
    current_time: &Duration,
    direction: Heading,
    animation: Option<&mut Animated<T>>,
) {
    if let Some((_, (animations, ref mut frame))) = animation {
        let frames = animations[direction as usize].frames.len();
        if frames == 0 {
            return;
        }

        let frame_duration = animation_time.as_millis() / frames as u128;
        let current_time = current_time.as_millis();

        *frame = (current_time / frame_duration) as usize % frames;
    }
}
