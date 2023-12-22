use std::time::Duration;

use definitions::heading::Heading;
use engine::{camera::Zoom, engine::GameEngine, input::keyboard::KeyCode};

use crate::{
    entity::{Entity, Movement},
    Finisterra,
};

impl Finisterra {
    pub fn process_input<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        let mut move_position = (0., 0.);
        if engine.key_held(KeyCode::ArrowRight) {
            self.entities[0].state.direction = Heading::East;
            move_position.0 = 1.;
        }

        if engine.key_held(KeyCode::ArrowLeft) {
            self.entities[0].state.direction = Heading::West;
            move_position.0 = -1.;
        }

        if engine.key_held(KeyCode::ArrowDown) {
            self.entities[0].state.direction = Heading::South;
            move_position.1 = -1.;
        }

        if engine.key_held(KeyCode::ArrowUp) {
            self.entities[0].state.direction = Heading::North;
            move_position.1 = 1.;
        }

        if move_position.0 != 0. || move_position.1 != 0. {
            let distance = 5. * delta.as_secs_f32();
            self.position.0 += move_position.0 * distance;
            self.position.1 += move_position.1 * distance;

            self.entities[0].position = [self.position.0, self.position.1];
            let world_x = ((self.position.0 * 32.) + 16.).floor();
            let world_y = (self.position.1 * 32.).floor();
            self.entities[0].world_position = [world_x, world_y];
        }

        if engine.key_pressed(KeyCode::Space) {
            self.entities[0].state.movement = match self.entities[0].state.movement {
                Movement::Idle => crate::entity::Movement::Walking {
                    animation_time: Duration::from_millis(500),
                    current_time: Duration::from_millis(0),
                },
                _ => Movement::Idle,
            };
        }

        if engine.key_released(KeyCode::KeyZ) {
            match engine.get_world_camera_zoom() {
                Zoom::None => engine.set_world_camera_zoom(Zoom::Double),
                Zoom::Double => engine.set_world_camera_zoom(Zoom::None),
            }
        }

        if engine.key_released(KeyCode::KeyN) {
            self.render_names = !self.render_names;
        }

        //
        // if is_key_pressed(KeyCode::Key1) {
        //     self.settings.draw_layer_0 = !self.settings.draw_layer_0;
        // }
        // if is_key_pressed(KeyCode::Key2) {
        //     self.settings.draw_layer_1 = !self.settings.draw_layer_1;
        // }
        // if is_key_pressed(KeyCode::Key3) {
        //     self.settings.draw_layer_2 = !self.settings.draw_layer_2;
        // }
        // if is_key_pressed(KeyCode::Key4) {
        //     self.settings.draw_layer_3 = !self.settings.draw_layer_3;
        // }
        // if is_key_pressed(KeyCode::U) {
        //     self.settings.draw_ui = !self.settings.draw_ui;
        // }
        // if is_key_pressed(KeyCode::M) {
        //     self.settings.cache_static_layers = !self.settings.cache_static_layers;
        //     self.screen_size_dirty = true;
        // }
        //
        // if is_key_pressed(KeyCode::N) {
        //     self.settings.draw_names = !self.settings.draw_names;
        // }
        //
        // if is_key_pressed(KeyCode::C) {
        //     self.settings.cache_entities = !self.settings.cache_entities;
        // }
        //
        // if is_key_pressed(KeyCode::A) {
        //     self.settings.use_atlases = !self.settings.use_atlases;
        // }
        //
        // if is_key_pressed(KeyCode::Delete) {
        //     // let mut textures = self.resources.textures.borrow_mut();
        //     // for (_, texture) in textures.drain() {
        //     //     texture.delete();
        //     // }
        // }
        //
        if engine.key_released(KeyCode::Space) {
            let entities_len = self.entities.len() - 1;
            for id in 1..=100 {
                let entity = Entity::random(1000000 + entities_len + id * 10, &self.resources);

                let [x, y] = entity.position;
                self.current_map.tiles[x as usize][y as usize].user = Some(entities_len + id);
                self.entities.push(entity);
            }
        }
    }
}
