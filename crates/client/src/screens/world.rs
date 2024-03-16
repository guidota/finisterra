use argentum::image::Image;
use engine::{
    camera::{self, Viewport, Zoom},
    draw::{image::DrawImage, Position, Target},
    engine::GameEngine,
    input::keyboard::KeyCode,
    CursorIcon,
};
use itertools::iproduct;
use nohash_hasher::IntMap;
use protocol::movement::{KeyPressed, KeyReleased};

use crate::{
    game::Context,
    ui::{colors::*, input_field::InputField},
    ui::{fonts::*, UI},
};

use self::{
    entity::{Character, Entity},
    hud::HUD,
};

use super::GameScreen;

pub mod entity;
pub mod hud;

pub struct WorldScreen {
    hud: HUD,

    entities: IntMap<u32, Entity>,

    me: u32,
    key_counter: u8,
}

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 540;

const WORLD_RENDER_WIDTH: u16 = 549; //17 * TILE_SIZE; // 17 tiles
const WORLD_RENDER_HEIGHT: u16 = 521; //16 * TILE_SIZE; // 16 tiles

impl WorldScreen {
    pub fn new<E: GameEngine>(
        context: &mut Context<E>,
        entity_id: u32,
        mut character: Character,
    ) -> Self {
        let ui = HUD::initialize(context, &character);
        let mut entities = IntMap::default();
        character.add_dialog(
            context.engine,
            "Rahma Nañarak O'al",
            TAHOMA_BOLD_8_SHADOW_ID,
            CYAN,
        );
        entities.insert(entity_id, Entity::Character(character));

        context.engine.set_mouse_cursor(CursorIcon::Default);

        Self {
            hud: ui,
            entities,
            me: entity_id,
            key_counter: 0,
        }
    }

    fn prepare_viewports<E: GameEngine>(&mut self, engine: &mut E) {
        let size = engine.get_window_size();
        let zoom = if size.height >= (SCREEN_HEIGHT * 2) && size.width >= (SCREEN_WIDTH * 2) {
            engine.set_camera_zoom(Zoom::Double);
            2
        } else {
            engine.set_camera_zoom(Zoom::None);
            1
        };

        let x_start = std::cmp::max(0, (size.width / zoom - SCREEN_WIDTH) / 2);
        let y_start = std::cmp::max(0, (size.height / zoom - SCREEN_HEIGHT) / 2);
        self.hud.x = x_start;
        self.hud.y = y_start;

        engine.set_ui_camera_viewport(Viewport {
            x: 0.,
            y: 0.,
            width: size.width as f32,
            height: size.height as f32,
        });

        // TODO: Review
        let world_camera_viewport = Viewport {
            x: (x_start as f32 + 9.) * zoom as f32,
            y: (y_start as f32 + 9.) * zoom as f32,
            width: WORLD_RENDER_WIDTH as f32 * zoom as f32,
            height: WORLD_RENDER_HEIGHT as f32 * zoom as f32,
        };
        engine.set_world_camera_viewport(world_camera_viewport);
    }

    fn update_character<E: GameEngine>(&mut self, context: &mut Context<'_, E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.update(context.engine);
            // camera follows main character
            let world_position = character.movement.world_position();
            context.engine.set_world_camera_position(camera::Position {
                x: (world_position.0 * 32.).floor() as f32,
                y: (world_position.1 * 32.).floor() as f32,
            });

            self.hud.update_character(context, character);
        }
    }

    fn update_fps<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let fps = format!("{:.0} FPS", 1. / context.engine.get_delta().as_secs_f64());
        self.hud.fps.set_text(&fps, context.engine);
    }

    fn update_ping<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let ping = context.connection.ping();
        self.hud
            .ping
            .set_text(&format!("PING: {ping}ms"), context.engine);
    }

    fn update_message_input<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let message_input_open = self.hud.message_input.is_some();
        let enter_pressed = context.engine.key_pressed(KeyCode::Enter);
        match (message_input_open, enter_pressed) {
            (true, true) => {
                if let Some(input) = self.hud.message_input.as_mut() {
                    let message = input.text();
                    if !message.is_empty() {
                        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me)
                        {
                            character.add_dialog(
                                context.engine,
                                message,
                                TAHOMA_BOLD_8_SHADOW_ID,
                                GRAY_6,
                            );
                        }
                    }
                }
                self.hud.message_input = None;
            }
            (false, true) => {
                let mut input_field = InputField::new(
                    GRAY_6,
                    GRAY_1,
                    (0, 0),
                    (200, 30),
                    TAHOMA_BOLD_8_SHADOW_ID,
                    context.resources.textures.input,
                    context,
                );
                input_field.focused = true;
                self.hud.message_input = Some(input_field);
            }
            _ => {}
        }
    }

    fn draw_entities<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            character.draw(context);
        }
    }

    fn draw_map<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            let position = &character.movement.tile_position();
            if let Some(map) = context.resources.maps.get(&(position.map as usize)) {
                let extra_tiles = 5;
                let x_start = (position.x - (17 / 2) - extra_tiles) as usize;
                let x_end = (position.x + (17 / 2) + extra_tiles) as usize;
                let y_start = (position.y - (16 / 2) - extra_tiles) as usize;
                let y_end = (position.y + (16 / 2) + extra_tiles) as usize;

                for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
                    let tile = &map.tiles[x][y];

                    let world_x = (x * 32) as u16;
                    let world_y = ((y * 32) - 32) as u16;

                    for layer in [0, 1, 2, 3].iter() {
                        if tile.graphics[*layer] != 0 {
                            let z = calculate_z(*layer, x as f32, y as f32);
                            let position = Position::new(world_x, world_y, z);
                            self.draw_grh(
                                context,
                                tile.graphics[*layer] as u32,
                                position,
                                Target::World,
                            );
                        }
                    }
                }
            }
        }
    }

    fn draw_grh<E: GameEngine>(
        &self,
        context: &mut Context<E>,
        image_id: u32,
        position: engine::draw::Position,
        target: Target,
    ) {
        let image = &context.resources.images[image_id as usize];
        self.draw_image(context, image, position, target);
    }

    fn draw_image<E: GameEngine>(
        &self,
        context: &mut Context<E>,
        image: &Image,
        mut position: engine::draw::Position,
        target: Target,
    ) {
        let image_num = image.file;
        position.x -= image.width / 2;

        context.engine.draw_image(
            DrawImage {
                position,
                color: [255, 255, 255, 255],
                source: [image.x, image.y, image.width, image.height],
                index: image_num,
            },
            target,
        );
    }

    fn process_input<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
            if context.engine.key_pressed(KeyCode::ArrowUp) {
                self.key_counter += 1;
                character.movement.key_pressed(KeyPressed {
                    key_id: self.key_counter,
                    direction: protocol::world::Direction::North,
                });
            }
            if context.engine.key_pressed(KeyCode::ArrowDown) {
                self.key_counter += 1;
                character.movement.key_pressed(KeyPressed {
                    key_id: self.key_counter,
                    direction: protocol::world::Direction::South,
                });
            }
            if context.engine.key_pressed(KeyCode::ArrowRight) {
                self.key_counter += 1;
                character.movement.key_pressed(KeyPressed {
                    key_id: self.key_counter,
                    direction: protocol::world::Direction::East,
                });
            }
            if context.engine.key_pressed(KeyCode::ArrowLeft) {
                self.key_counter += 1;
                character.movement.key_pressed(KeyPressed {
                    key_id: self.key_counter,
                    direction: protocol::world::Direction::West,
                });
            }
            if context.engine.key_released(KeyCode::ArrowUp) {
                character.movement.key_released(KeyReleased {
                    direction: protocol::world::Direction::North,
                });
            }
            if context.engine.key_released(KeyCode::ArrowDown) {
                character.movement.key_released(KeyReleased {
                    direction: protocol::world::Direction::South,
                });
            }
            if context.engine.key_released(KeyCode::ArrowLeft) {
                character.movement.key_released(KeyReleased {
                    direction: protocol::world::Direction::West,
                });
            }
            if context.engine.key_released(KeyCode::ArrowRight) {
                character.movement.key_released(KeyReleased {
                    direction: protocol::world::Direction::East,
                });
            }
        }

        if context.engine.key_pressed(KeyCode::KeyH) {
            if let Some(Entity::Character(character)) = self.entities.get_mut(&self.me) {
                character.add_dialog(
                    context.engine,
                    "Rahma Nañarak O'al",
                    TAHOMA_BOLD_8_SHADOW_ID,
                    CYAN,
                );
            }
        }
    }
}

impl GameScreen for WorldScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.process_input(context);

        self.prepare_viewports(context.engine);
        self.hud.update(context);

        self.update_fps(context);
        self.update_ping(context);
        self.update_character(context);
        self.update_message_input(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.hud.draw(context);

        self.draw_map(context);
        self.draw_entities(context);
    }
}

fn calculate_z(layer: usize, x: f32, y: f32) -> f32 {
    match layer {
        0 => 0.,
        3 => 1.,
        i => (i as f32 * 1000. + (100. - y) * 10. - x) / 4000.,
    }
}
