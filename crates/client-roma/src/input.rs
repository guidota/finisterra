use std::time::Duration;

use roma::{event::VirtualKeyCode, graphics::vec2::vec2, Roma};

use crate::Finisterra;

impl Finisterra {
    pub fn process_input(&mut self, roma: &mut Roma, delta: Duration) {
        let mut move_position = vec2(0., 0.);
        if roma.input.key_held(VirtualKeyCode::Right) {
            move_position.x = 1.;
        }

        if roma.input.key_held(VirtualKeyCode::Left) {
            move_position.x = -1.;
        }

        if roma.input.key_held(VirtualKeyCode::Down) {
            move_position.y = -1.;
        }

        if roma.input.key_held(VirtualKeyCode::Up) {
            move_position.y = 1.;
        }

        if move_position.x != 0. || move_position.y != 0. {
            let distance = 5. * delta.as_secs_f32();
            self.position.x += move_position.x * distance;
            self.position.y += move_position.y * distance;
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