use std::{cmp::min, ops::Range};

use ao::ao_20::graphics::Image;
use macroquad::prelude::*;

use crate::error::RuntimeError;

use super::Game;

impl Game {
    pub async fn draw_layer(&mut self, layer: usize, x_range: Range<usize>, y_range: Range<usize>) {
        for y in y_range {
            for x in x_range.clone() {
                let tile = self.map.tiles[x][y];
                if tile.graphics[layer] != 0 {
                    _ = self
                        .draw_grh(
                            &tile.graphics[layer].to_string(),
                            (x * 32) as isize,
                            (y * 32) as isize,
                        )
                        .await;
                }
                if layer == 2 && tile.char_index > 0 {
                    self.draw_entity(tile.char_index).await;
                }
            }
        }
    }

    pub async fn prepare_and_draw_layer(&mut self, layer: usize, x: usize, y: usize) {
        if !self.map_render_targets[layer].1 || self.screen_size_dirty {
            self.map_static_camera.render_target = Some(self.map_render_targets[layer].0);
            set_camera(&self.map_static_camera);
            let x_range = 0..self.map.tiles.len();
            let y_range = 0..self.map.tiles.len();
            self.draw_layer(layer, x_range, y_range).await;
            self.map_render_targets[layer].1 = true;
            set_camera(&self.world_camera);
        }

        let (y_start, y_end) = (y.saturating_sub(15), y + 15);
        let (x_start, x_end) = (x.saturating_sub(15), x + 15);

        let vision_range = Rect::new(
            x_start as f32 * 32.,
            y_start as f32 * 32.,
            (x_end - x_start) as f32 * 32.,
            (y_end - y_start) as f32 * 32.,
        );
        draw_texture_ex(
            self.map_render_targets[layer].0.texture,
            vision_range.x,
            vision_range.y,
            WHITE,
            DrawTextureParams {
                source: Some(vision_range),
                ..Default::default()
            },
        );
    }

    pub async fn draw_world(&mut self) {
        let (x, y) = (
            self.position.x.floor() as usize,
            self.position.y.floor() as usize,
        );
        self.prepare_and_draw_layer(0, x, y).await;

        let (y_start, y_end) = (y.saturating_sub(15), min(y + 15, 100));
        let (x_start, x_end) = (x.saturating_sub(15), min(x + 15, 100));
        self.draw_layer(1, x_start..x_end, y_start..y_end).await;
        self.draw_layer(2, x_start..x_end, y_start..y_end).await;

        // self.prepare_and_draw_layer(2, x, y).await;
        self.prepare_and_draw_layer(3, x, y).await;
    }

    pub fn draw_fps(&self) {
        draw_text_ex(
            format!("Pandora: {}", get_fps()).as_str(),
            5.,
            15.,
            TextParams {
                font: self.resources.fonts.tahoma,
                font_size: 12,
                color: GREEN,
                ..Default::default()
            },
        );
    }

    pub fn draw_position(&self) {
        draw_text_ex(
            format!("X:{:.0}-Y:{:.0}", self.position.x, self.position.y).as_str(),
            5.,
            25.,
            TextParams {
                font: self.resources.fonts.tahoma,
                font_size: 10,
                color: YELLOW,
                ..Default::default()
            },
        );
    }

    pub fn draw_interface(&self) {
        draw_texture(self.resources.interface.main, 0., 0., WHITE);
    }

    pub async fn draw_animated_grh(
        &self,
        id: usize,
        x: isize,
        y: isize,
    ) -> Result<(), RuntimeError> {
        if let Ok(animation) = self.resources.get_animation(id.to_string().as_str()) {
            return self.draw_grh(&animation.frames[0], x, y).await;
        } else {
            println!("animation not found {id}");
        }

        Err(RuntimeError::AnimationNotFound)
    }

    pub async fn draw_grh(&self, id: &str, x: isize, y: isize) -> Result<(), RuntimeError> {
        if let Ok(image) = self.resources.get_image(id) {
            return self.draw_image(image, x, y).await;
        }

        Err(RuntimeError::GrhNotFound)
    }

    pub async fn draw_image(&self, image: &Image, x: isize, y: isize) -> Result<(), RuntimeError> {
        let texture = self.resources.get_texture(image.file_num as usize).await?;
        let x = x - (image.width / 2) as isize;
        let y = y - (image.height) as isize;

        draw_texture_ex(
            texture,
            x as f32,
            y as f32,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    image.x as f32,
                    image.y as f32,
                    image.width as f32,
                    image.height as f32,
                )),
                ..Default::default()
            },
        );
        Ok(())
    }
}
