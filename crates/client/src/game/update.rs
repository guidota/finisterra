use macroquad::prelude::*;

use super::Game;

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
