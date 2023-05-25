use macroquad::prelude::*;

use super::Game;

impl Game {
    pub fn update_position(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.position.x += 0.01;
        }
        if is_key_down(KeyCode::Left) {
            self.position.x -= 0.01;
        }
        if is_key_down(KeyCode::Down) {
            self.position.y += 0.01;
        }
        if is_key_down(KeyCode::Up) {
            self.position.y -= 0.01;
        }

        self.position.x = self.position.x.clamp(10., 90.);
        self.position.y = self.position.y.clamp(10., 90.);

        self.world_camera.target = vec2(
            (self.position.x * 32.).floor(),
            (self.position.y * 32.).floor(),
        );
    }
}
