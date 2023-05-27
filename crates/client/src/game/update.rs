use macroquad::prelude::*;

use super::{entity::Entity, Game};

impl Game {
    pub fn update_position(&mut self) {
        // delta time
        let distance = 1. / 0.250 * get_frame_time();
        if is_key_down(KeyCode::Right) {
            self.position.x += distance;
        }
        if is_key_down(KeyCode::Left) {
            self.position.x -= distance;
        }
        if is_key_down(KeyCode::Down) {
            self.position.y += distance;
        }
        if is_key_down(KeyCode::Up) {
            self.position.y -= distance;
        }

        if is_key_pressed(KeyCode::Key1) {
            self.settings.draw_layer_0 = !self.settings.draw_layer_0;
        }
        if is_key_pressed(KeyCode::Key2) {
            self.settings.draw_layer_1 = !self.settings.draw_layer_1;
        }
        if is_key_pressed(KeyCode::Key3) {
            self.settings.draw_layer_2 = !self.settings.draw_layer_2;
        }
        if is_key_pressed(KeyCode::Key4) {
            self.settings.draw_layer_3 = !self.settings.draw_layer_3;
        }
        if is_key_pressed(KeyCode::U) {
            self.settings.draw_ui = !self.settings.draw_ui;
        }
        if is_key_pressed(KeyCode::M) {
            self.settings.cache_static_layers = !self.settings.cache_static_layers;
            self.screen_size_dirty = true;
        }

        if is_key_pressed(KeyCode::N) {
            self.settings.draw_names = !self.settings.draw_names;
        }

        if is_key_pressed(KeyCode::C) {
            self.settings.cache_entities = !self.settings.cache_entities;
            self.screen_size_dirty = true;
        }

        if is_key_pressed(KeyCode::A) {
            self.settings.use_atlases = !self.settings.use_atlases;
            self.screen_size_dirty = true;
        }

        if is_key_pressed(KeyCode::Delete) {
            print!("Deleting textures...");
            let mut textures = self.resources.textures.borrow_mut();
            for (_, texture) in textures.drain() {
                texture.delete();
            }
            println!(" Ok");
        }

        if is_key_pressed(KeyCode::B) {
            build_textures_atlas();
        }

        if is_key_pressed(KeyCode::Space) {
            let entities_len = self.entities.len();
            for id in 1..=100 {
                let random = Entity::random(&self.resources);
                let Vec2 { x, y } = random.position;
                self.map.tiles[x as usize][y as usize].char_index = id + entities_len;
                self.entities.insert(id + entities_len, random);
            }
        }

        self.position.x = self.position.x.clamp(8., 92.);
        self.position.y = self.position.y.clamp(8., 92.);

        let previous_position = self.entities.get_mut(&1).unwrap().position;
        let previous_x = previous_position.x.floor() as usize;
        let previous_y = previous_position.y.floor() as usize;
        self.entities.get_mut(&1).unwrap().position = self.position;

        let new_position = self.entities.get_mut(&1).unwrap().position;
        let new_x = new_position.x.floor() as usize;
        let new_y = new_position.y.floor() as usize;

        if new_x != previous_x || new_y != previous_y {
            self.map.tiles[new_x][new_y].char_index = 1;
            self.map.tiles[previous_x][previous_y].char_index = 0;
        }

        self.world_camera.target = vec2(
            (self.position.x * 32.).floor(),
            (self.position.y * 32.).floor(),
        );
    }
}
