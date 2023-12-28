use std::{ops::Add, rc::Rc, time::Duration};

use definitions::{
    animation::Animation, body::Body, client::ClientResources, gear::HeadGear, head::Head,
    heading::Heading, image::Image, shield::Shield, weapon::Weapon,
};
use engine::{
    draw::{
        image::DrawImage,
        text::{DrawText, Orientation, ParsedText},
        Dimensions, Position, Target,
    },
    engine::GameEngine,
};
use rand::{seq::IteratorRandom, Rng};
use rnglib::{Language, RNG};

use crate::{
    z_ordering::{get_headgear_z, get_shield_z, get_weapon_z},
    Finisterra, RenderTarget, HALF_TILE, TILE_SIZE, Z_ORDERING,
};

#[derive(Debug)]
pub enum Movement {
    Idle,
    Walking {
        animation_time: Duration,
        current_time: Duration,
    },
}

type Animated<T> = (T, ([Rc<Animation>; 4], usize));

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub body: Option<Animated<Body>>,
    pub head: Option<(Head, [Rc<Image>; 4])>,
    pub weapon: Option<Animated<Weapon>>,
    pub shield: Option<Animated<Shield>>,
    pub head_gear: Option<(HeadGear, [Rc<Image>; 4])>,
    pub name: (String, ParsedText),
    pub position: [f32; 2],
    pub world_position: [f32; 2],

    // state
    pub direction: Heading,
    pub movement: Movement,
    pub invisible: bool,

    // rendering
    pub render_target: RenderTarget,
    pub texture_dimensions: Dimensions,
}

lazy_static::lazy_static! {
    static ref NAME_GENERATOR: RNG = RNG::from(&Language::Fantasy);
}

impl Entity {
    pub fn random<E: GameEngine>(engine: &mut E, id: usize, resources: &ClientResources) -> Self {
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

        let mut retries = 0;
        let head = loop {
            if retries >= 3 {
                break None;
            }
            let (_, head) = resources.heads.iter().choose(&mut rng).unwrap();
            if head.images[..4].iter().any(|image| image == &0) {
                retries += 1;
                continue;
            }
            let images = [
                resources.images[head.images[0]].clone().unwrap(),
                resources.images[head.images[1]].clone().unwrap(),
                resources.images[head.images[2]].clone().unwrap(),
                resources.images[head.images[3]].clone().unwrap(),
            ];
            break Some((head.clone(), images));
        };

        let mut retries = 0;

        let weapon = loop {
            if retries >= 3 {
                break None;
            }

            let (_, weapon) = resources.weapons.iter().choose(&mut rng).unwrap();
            let Some(animations) = get_animations(weapon.animations) else {
                retries += 1;
                continue;
            };
            break Some((weapon.clone(), animations));
        };

        let mut retries = 0;
        let shield = loop {
            if retries >= 3 {
                break None;
            }
            let (_, shield) = resources.shields.iter().choose(&mut rng).unwrap();
            let Some(animations) = get_animations(shield.animations) else {
                retries += 1;
                continue;
            };
            break Some((shield.clone(), animations));
        };

        let mut retries = 0;
        let head_gear = loop {
            if retries >= 3 {
                break None;
            }
            let (_, head) = resources.headgears.iter().choose(&mut rng).unwrap();
            if head.images[..4].iter().any(|image| image == &0) {
                retries += 1;
                continue;
            }
            let default = resources.images[1].clone().unwrap();
            let images = [
                resources.images[head.images[0]]
                    .clone()
                    .unwrap_or(default.clone()),
                resources.images[head.images[1]]
                    .clone()
                    .unwrap_or(default.clone()),
                resources.images[head.images[2]]
                    .clone()
                    .unwrap_or(default.clone()),
                resources.images[head.images[3]]
                    .clone()
                    .unwrap_or(default.clone()),
            ];
            break Some((head.clone(), images));
        };

        let x = rng.gen_range(0..100) as f32;
        let y = rng.gen_range(0..100) as f32;

        let name = NAME_GENERATOR.generate_name();
        let parsed_text = engine.parse_text(1, &name, Orientation::Center).unwrap();

        let dimensions = Self::calculate_render_target_dimensions(&body_animations, resources);
        Self {
            id,
            body: Some((random_body, (body_animations, 0))),
            head,
            weapon: weapon
                .map(|(random_weapon, weapon_animations)| (random_weapon, (weapon_animations, 0))),
            shield: shield
                .map(|(random_shield, shield_animations)| (random_shield, (shield_animations, 0))),
            head_gear,
            position: [x, y],
            world_position: [
                x * TILE_SIZE as f32 + HALF_TILE as f32,
                y * TILE_SIZE as f32,
            ],
            name: (name, parsed_text),
            movement: if rng.gen_bool(0.5) {
                Movement::Walking {
                    animation_time: Duration::from_millis(500),
                    current_time: Duration::from_millis(0),
                }
            } else {
                Movement::Idle
            },
            direction,
            invisible: false,
            render_target: RenderTarget::Uninitialized,
            texture_dimensions: dimensions,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if let Movement::Walking {
            animation_time,
            ref mut current_time,
        } = &mut self.movement
        {
            *current_time = current_time.add(delta);
            if current_time > animation_time {
                *current_time = Duration::from_millis(0);
            }

            let direction = self.direction;
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

    /// Calculates based on the body, the required dimensions to draw all frames
    fn calculate_render_target_dimensions(
        body_animations: &[Rc<Animation>; 4],
        resources: &ClientResources,
    ) -> Dimensions {
        let mut images = vec![];
        for animation in body_animations {
            for frame in &animation.frames {
                if let Some(image) = &resources.images[*frame as usize] {
                    images.push(image);
                }
            }
        }

        let max_width = images.iter().max_by_key(|image| image.width).unwrap().width;
        let width = match max_width * 6 {
            0..=255 => 256,
            256..=511 => 512,
            512..=1023 => 1024,
            _ => 4096,
        };
        let max_height = images
            .iter()
            .max_by_key(|image| image.height)
            .unwrap()
            .height;
        // room for head and helmet
        let max_height = max_height + 25;
        let height = match max_height * 4 {
            0..=255 => 256,
            256..=511 => 512,
            512..=1023 => 1024,
            _ => 4096,
        };

        Dimensions { width, height }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position[0] = x;
        self.position[1] = y;
        self.world_position[0] = x * TILE_SIZE as f32 + HALF_TILE as f32;
        self.world_position[1] = y * TILE_SIZE as f32;
    }

    /// Draws every frame to the render target textrue
    fn prepare_texture<E: GameEngine>(
        &mut self,
        engine: &mut E,
        resources: &ClientResources,
    ) -> u64 {
        let texture_id = match self.render_target {
            RenderTarget::Ready { texture_id } => {
                return texture_id;
            }
            RenderTarget::Uninitialized => {
                let texture_id = engine.create_texture(self.texture_dimensions);
                self.render_target = RenderTarget::Dirty { texture_id };
                texture_id
            }
            RenderTarget::Dirty { texture_id } => texture_id,
        };

        if let Some((body, (animations, _))) = &self.body {
            let z = 0.5;

            let frame_width = self.texture_dimensions.width / 6;
            let frame_height = self.texture_dimensions.height / 4;
            let color = [255, 255, 255, 255];

            let mut y = 0;
            for heading in Heading::iterator() {
                let mut x = 0;
                let animation = &animations[(*heading) as usize];
                let frames = animation.frames.len();
                for frame in 0..frames {
                    let image_id = animation.frames[frame];
                    if let Some(image) = &resources.images[image_id as usize] {
                        let mut position = Position { x, y, z };
                        let image_num = image.file_num;
                        position.x += (frame_width / 2) - (image.width / 2);

                        engine.draw_image(
                            image_num,
                            DrawImage {
                                position,
                                color,
                                source: [image.x, image.y, image.width, image.height],
                                index: image_num as u32,
                            },
                            Target::Texture { id: texture_id },
                        );
                    }

                    let head_offset = &body.head_offset;
                    if let Some((_, images)) = &self.head {
                        let image = &images[(*heading) as usize];
                        let image_num = image.file_num;

                        let mut position = Position::new(x, y, z);
                        position.x += (frame_width / 2) - (image.width / 2);
                        position.x -= head_offset.x as u16;
                        position.y -= head_offset.y as u16;

                        engine.draw_image(
                            image_num,
                            DrawImage {
                                position,
                                color,
                                source: [image.x, image.y, image.width, image.height],
                                index: image_num as u32,
                            },
                            Target::Texture { id: texture_id },
                        );
                    }

                    if let Some((_, images)) = &self.head_gear {
                        let image = &images[(*heading) as usize];
                        let image_num = image.file_num;

                        let mut position = Position::new(x, y, z);
                        position.x += (frame_width / 2) - (image.width / 2);
                        position.x -= head_offset.x as u16;
                        position.y -= head_offset.y as u16;
                        position.z = get_headgear_z(z, *heading);

                        engine.draw_image(
                            image_num,
                            DrawImage {
                                position,
                                color,
                                source: [image.x, image.y, image.width, image.height],
                                index: image_num as u32,
                            },
                            Target::Texture { id: texture_id },
                        );
                    }

                    if let Some((_, (animations, _))) = &self.weapon {
                        let animation = &animations[(*heading) as usize];
                        if frame < animation.frames.len() {
                            let image_id = animation.frames[frame];
                            if let Some(image) = &resources.images[image_id as usize] {
                                let mut position = Position { x, y, z };
                                let image_num = image.file_num;
                                position.x += (frame_width / 2) - (image.width / 2);
                                position.z = get_weapon_z(z, *heading);

                                engine.draw_image(
                                    image_num,
                                    DrawImage {
                                        position,
                                        color,
                                        source: [image.x, image.y, image.width, image.height],
                                        index: image_num as u32,
                                    },
                                    Target::Texture { id: texture_id },
                                );
                            }
                        }
                    }

                    if let Some((_, (animations, _))) = &self.shield {
                        let animation = &animations[(*heading) as usize];

                        if frame < animation.frames.len() {
                            let image_id = animation.frames[frame];
                            if let Some(image) = &resources.images[image_id as usize] {
                                let mut position = Position { x, y, z };
                                let image_num = image.file_num;
                                position.x += (frame_width / 2) - (image.width / 2);
                                position.z = get_shield_z(z, *heading);

                                engine.draw_image(
                                    image_num,
                                    DrawImage {
                                        position,
                                        color,
                                        source: [image.x, image.y, image.width, image.height],
                                        index: image_num as u32,
                                    },
                                    Target::Texture { id: texture_id },
                                );
                            }
                        }
                    }

                    x += frame_width;
                }
                y += frame_height;
            }
        }
        self.render_target = RenderTarget::Ready { texture_id };
        texture_id
    }

    pub fn draw<E: GameEngine>(&mut self, engine: &mut E, resources: &ClientResources) {
        if self.should_draw_to_texture() {
            let texture_id = self.prepare_texture(engine, resources);
            let [x, y] = self.position;
            let [world_x, world_y] = self.world_position;
            let z = Z_ORDERING[2][x as usize][y as usize];
            let frame_width = self.texture_dimensions.width / 6;
            let frame_height = self.texture_dimensions.height / 4;

            let (image_x, image_y) = self.get_current_frame_offsets(frame_width, frame_height);
            let color = self.body_color();
            engine.draw_image(
                texture_id,
                DrawImage {
                    position: Position::new(
                        world_x as u16 - (TILE_SIZE / 2) as u16 - (frame_width / 2),
                        world_y as u16,
                        z,
                    ),
                    color,
                    source: [image_x, image_y, frame_width, frame_height],
                    index: texture_id as u32,
                },
                Target::World,
            );
        } else {
            // draw current frame to world
            let [x, y] = self.position;
            let [world_x, world_y] = self.world_position;
            let world_x = world_x as u16 - (TILE_SIZE / 2) as u16;
            let world_y = world_y as u16;
            let z = Z_ORDERING[2][x as usize][y as usize];

            let heading = self.direction as usize;
            if let Some((body, (animations, frame))) = &self.body {
                self.draw_animation(
                    engine,
                    resources,
                    &animations[heading],
                    *frame,
                    Position::new(world_x, world_y, z),
                );
                let head_offset = &body.head_offset;
                if let Some((_, images)) = &self.head {
                    let x = (world_x as isize - head_offset.x) as u16;
                    let y = (world_y as isize - head_offset.y) as u16;
                    let image = &images[heading];
                    self.draw_image(engine, image, Position::new(x, y, z));
                }

                if let Some((_, images)) = &self.head_gear {
                    let x = (world_x as isize - head_offset.x) as u16;
                    let y = (world_y as isize - head_offset.y) as u16;
                    let image = &images[heading];
                    self.draw_image(
                        engine,
                        image,
                        Position::new(x, y, get_headgear_z(z, self.direction)),
                    );
                }

                if let Some((_, (animations, frame))) = &self.weapon {
                    self.draw_animation(
                        engine,
                        resources,
                        &animations[heading],
                        *frame,
                        Position::new(world_x, world_y, get_weapon_z(z, self.direction)),
                    );
                }
                if let Some((_, (animations, frame))) = &self.shield {
                    self.draw_animation(
                        engine,
                        resources,
                        &animations[heading],
                        *frame,
                        Position::new(world_x, world_y, get_shield_z(z, self.direction)),
                    );
                }
            }
        }
    }

    fn draw_grh<E: GameEngine>(
        &self,
        engine: &mut E,
        resources: &ClientResources,
        image_id: u32,
        position: Position,
    ) {
        if let Some(image) = &resources.images[image_id as usize] {
            self.draw_image(engine, image, position);
        }
    }

    fn draw_image<E: GameEngine>(&self, engine: &mut E, image: &Image, mut position: Position) {
        let image_num = image.file_num;
        position.x -= (image.width as f32 / 2.) as u16;

        engine.draw_image(
            image_num,
            DrawImage {
                position,
                color: self.body_color(),
                source: [image.x, image.y, image.width, image.height],
                index: image_num as u32,
            },
            Target::World,
        );
    }

    fn draw_animation<E: GameEngine>(
        &self,
        engine: &mut E,
        resources: &ClientResources,
        animation: &Animation,
        frame: usize,
        position: Position,
    ) {
        if animation.frames.is_empty() {
            return;
        }
        self.draw_grh(engine, resources, animation.frames[frame], position);
    }

    pub fn draw_name<E: GameEngine>(&self, engine: &mut E) {
        let color = self.name_color();
        let [x, y] = self.position;
        let [world_x, world_y] = self.world_position;
        let z = Z_ORDERING[2][x as usize][y as usize];

        engine.draw_text(
            Finisterra::TAHOMA_ID,
            DrawText {
                text: &self.name.1,
                position: Position {
                    x: world_x as u16 - (TILE_SIZE / 2) as u16,
                    y: world_y as u16 - 10,
                    z,
                },
                color,
            },
            Target::World,
        );
    }

    pub fn should_draw_to_texture(&self) -> bool {
        self.invisible
    }

    fn name_color(&self) -> engine::draw::Color {
        if self.invisible {
            [171, 139, 98, 150]
        } else {
            [0, 128, 255, 255]
        }
    }

    fn body_color(&self) -> engine::draw::Color {
        if self.invisible {
            [200, 200, 200, 120]
        } else {
            [255, 255, 255, 255]
        }
    }

    fn get_current_frame_offsets(&self, frame_width: u16, frame_height: u16) -> (u16, u16) {
        if let Some((_, (_, frame))) = self.body {
            let x = frame as u16 * frame_width;
            let y = (3 - (self.direction as u16)) * frame_height;
            return (x, y);
        }
        (0, 0)
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
