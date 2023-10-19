use std::time::Duration;

use definitions::heading::Heading;
use roma::{get_delta, get_input, VirtualKeyCode};

use crate::{entity::Movement, Finisterra};

impl Finisterra {
    pub fn process_input(&mut self) {
        let delta = get_delta();
        let input = get_input();
        let mut move_position = (0., 0.);
        if input.key_held(VirtualKeyCode::Right) {
            self.entities[0].state.direction = Heading::East;
            move_position.0 = 1.;
        }

        if input.key_held(VirtualKeyCode::Left) {
            self.entities[0].state.direction = Heading::West;
            move_position.0 = -1.;
        }

        if input.key_held(VirtualKeyCode::Down) {
            self.entities[0].state.direction = Heading::South;
            move_position.1 = -1.;
        }

        if input.key_held(VirtualKeyCode::Up) {
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

        if input.key_pressed(VirtualKeyCode::Space) {
            self.entities[0].state.movement = match self.entities[0].state.movement {
                Movement::Idle => crate::entity::Movement::Walking {
                    animation_time: Duration::from_millis(500),
                    current_time: Duration::from_millis(0),
                },
                _ => Movement::Idle,
            };
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
        // if is_key_pressed(KeyCode::Space) {
        //     // let entities_len = self.entities.len();
        //     // for id in 1..=100 {
        //     //     let random = Entity::random(&self.resources);
        //     //     let Vec2 { x, y } = random.position;
        //     //     self.map.tiles[x as usize][y as usize].char_index = id + entities_len;
        //     //     self.entities.insert(id + entities_len, random);
        // }
    }
}
