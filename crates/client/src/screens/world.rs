use std::{collections::VecDeque, time::Instant};

use engine::{
    camera::{self, Viewport, Zoom},
    engine::GameEngine,
    CursorIcon,
};
use nohash_hasher::IntMap;
use shared::{
    protocol::server::{CharacterUpdate, DialogKind, ServerPacket},
    world::{Direction, WorldPosition},
};

use crate::{
    game::Context,
    screens::world::map::WorldMap,
    ui::{colors::*, fonts::*},
};

use self::{
    entity::{Character, Entity},
    fps::Fps,
    hud::HUD,
};

use super::GameScreen;

mod depth;
pub mod entity;
pub mod fps;
pub mod hud;
pub mod input;
pub mod interpolation;
mod map;
pub mod prediction;
pub mod reconciliation;

const TILE_SIZE: u16 = 32;
const TILE_SIZE_F: f32 = TILE_SIZE as f32;

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 540;
const WORLD_RENDER_WIDTH: u16 = 549; // It's around 17 tiles
const WORLD_RENDER_HEIGHT: u16 = 521; // It's around 16 tiles
const HORIZONTAL_TILES: u16 = 17;
const VERTICAL_TILES: u16 = 16;

pub struct WorldScreen {
    hud: HUD,

    entities: IntMap<u32, Entity>,

    // client entity
    entity_id: u32,

    // client side prediction
    input: VecDeque<Direction>,
    movement_sequence: u8,
    predictions: Vec<(u8, WorldPosition)>,
    last_move: Instant,
    map: WorldMap,
    fps: Fps,
}

impl GameScreen for WorldScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.process_messages(context);
        self.process_input(context);

        self.prepare_viewports(context.engine);

        self.update_hud(context);
        self.update_fps(context);
        self.update_ping(context);
        self.update_character(context);
        self.update_message_input(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.draw_hud(context);
        self.draw_world_2(context);
    }
}

impl WorldScreen {
    pub fn new<E: GameEngine>(
        context: &mut Context<E>,
        entity_id: u32,
        character: Character,
    ) -> Self {
        let ui = HUD::initialize(context, &character);
        let mut entities = IntMap::default();
        let map = context.maps.get(&character.position.map);
        map.tile_mut(character.position.x, character.position.y)
            .user = Some(entity_id);
        entities.insert(entity_id, Entity::Character(character));

        context.engine.set_mouse_cursor(CursorIcon::Default);

        Self {
            hud: ui,
            entities,
            entity_id,
            movement_sequence: 0,
            predictions: vec![],
            input: VecDeque::new(),
            last_move: Instant::now(),
            fps: Fps::default(),
            map: WorldMap::initialize(context),
            // map: WorldMap::default(),
        }
    }

    fn process_messages<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let messages = context.connection.read();
        for message in messages {
            self.process_message(message, context);
        }
    }

    pub fn process_message<E: GameEngine>(
        &mut self,
        message: ServerPacket,
        context: &mut Context<E>,
    ) {
        match message {
            ServerPacket::Intervals => todo!(),
            ServerPacket::Connection(_) => todo!(),
            ServerPacket::Account(_) => todo!(),
            ServerPacket::CharacterUpdate(update) => match update {
                CharacterUpdate::Create {
                    entity_id,
                    character,
                } => {
                    let map = context.maps.get(&character.position.map);
                    map.tile_mut(character.position.x, character.position.y)
                        .user = Some(entity_id);

                    let entity = Entity::Character(Character::from(context, character));

                    self.entities.insert(entity_id, entity);
                }
                CharacterUpdate::Remove { entity_id } => {
                    self.entities.remove(&entity_id);
                }
                CharacterUpdate::Move {
                    entity_id,
                    position,
                } => {
                    let Some(Entity::Character(character)) = self.entities.get_mut(&entity_id)
                    else {
                        return;
                    };

                    character.move_to(position);
                }
                CharacterUpdate::Translate {
                    entity_id,
                    position,
                } => {
                    if entity_id == self.entity_id {
                        self.predictions.clear();
                    }

                    let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id)
                    else {
                        return;
                    };
                    character.translate(position);
                }
                CharacterUpdate::MoveResponse {
                    request_id,
                    position,
                } => {
                    self.reconciliation(request_id, position, context);
                }
                CharacterUpdate::Heading {
                    entity_id,
                    direction,
                } => {
                    let Some(Entity::Character(character)) = self.entities.get_mut(&entity_id)
                    else {
                        return;
                    };
                    character.animation.change_direction(direction);
                }
                CharacterUpdate::DialogAdd {
                    entity_id,
                    text,
                    kind,
                } => {
                    let Some(Entity::Character(character)) = self.entities.get_mut(&entity_id)
                    else {
                        return;
                    };
                    let (font_id, color) = match kind {
                        DialogKind::Normal => (TAHOMA_BOLD_8_SHADOW_ID, WHITE),
                        DialogKind::Shout => (TAHOMA_BOLD_8_SHADOW_ID, RED),
                        DialogKind::Role => (TAHOMA_BOLD_8_SHADOW_ID, YELLOW),
                        DialogKind::MagicWords => (TAHOMA_BOLD_8_SHADOW_ID, CYAN),
                    };
                    character.add_dialog(context.engine, &text, font_id, color);
                }
                _ => {}
            },
            ServerPacket::UserUpdate(_) => todo!(),
            ServerPacket::Event(_) => todo!(),
            ServerPacket::Object(_) => todo!(),
            ServerPacket::Message(_) => todo!(),
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

    fn update_character<E: GameEngine>(&mut self, context: &mut Context<E>) {
        for (id, entity) in self.entities.iter_mut() {
            match entity {
                Entity::Character(character) => {
                    if character.just_started_moving() {
                        let old_position = character.position;
                        if let Some(new_position) = character.position_buffer.first() {
                            let map = context.maps.get(&old_position.map);
                            map.tile_mut(old_position.x, old_position.y).user = None;
                            let map = context.maps.get(&new_position.map);
                            map.tile_mut(new_position.x, new_position.y).user = Some(*id);
                        }
                    }
                }
            }
            entity.update(context.engine);
        }
        if let Some(Entity::Character(character)) = self.entities.get_mut(&self.entity_id) {
            // camera follows main character
            let x = character.render_position.0;
            let y = character.render_position.1 + TILE_SIZE_F / 2.;
            context
                .engine
                .set_world_camera_position(camera::Position { x, y });

            self.hud.update_character(context, character);

            if !self.input.is_empty() && (!character.moving() || character.just_finished_moving()) {
                self.start_move(context);
            }
        }
    }

    fn update_fps<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.fps.update(context.engine.get_delta());
        let fps = format!("{:.0} FPS", self.fps.get());
        self.hud.fps.set_text(&fps, context.engine);
    }

    fn update_ping<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let ping = context.connection.ping();
        self.hud.ping.set_text(&format!("{ping}ms"), context.engine);
    }
}

fn get_range(
    position: &WorldPosition,
    extra_tiles_h: u16,
    extra_tiles_v: u16,
) -> (usize, usize, usize, usize) {
    let x_start = (position.x - extra_tiles_h) as usize;
    let x_end = (position.x + extra_tiles_h) as usize;
    let y_start = (position.y - extra_tiles_v) as usize;
    let y_end = (position.y + extra_tiles_v) as usize;
    (x_start, x_end, y_start, y_end)
}
