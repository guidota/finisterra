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
                            (x * 32) as f32,
                            (y * 32) as f32,
                            1.,
                            Some(layer),
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
            self.map_static_camera
                .render_target
                .unwrap()
                .texture
                .set_filter(FilterMode::Nearest);
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
        let (y_start, y_end) = (y.saturating_sub(12), min(y + 12, 100));
        let (x_start, x_end) = (x.saturating_sub(12), min(x + 12, 100));

        if self.settings.draw_layer_0 {
            if self.settings.cache_static_layers {
                self.prepare_and_draw_layer(0, x, y).await;
            } else {
                self.draw_layer(0, x_start..x_end, y_start..y_end).await;
            }
        }
        if self.settings.draw_layer_1 {
            self.draw_layer(1, x_start..x_end, y_start..y_end).await;
        }

        if self.settings.draw_layer_2 {
            self.draw_layer(2, x_start..x_end, y_start..y_end).await;
        }

        if self.settings.draw_layer_3 {
            if self.settings.cache_static_layers {
                self.prepare_and_draw_layer(3, x, y).await;
            } else {
                self.draw_layer(3, x_start..x_end, y_start..y_end).await;
            }
        }
    }

    pub fn draw_fps(&self) {
        draw_text_ex(
            format!("FPS: {}", get_fps()).as_str(),
            5.,
            15.,
            TextParams {
                font: self.resources.fonts.tahoma,
                font_size: 12,
                color: GREEN,
                ..Default::default()
            },
        );
        draw_text_ex(
            format!("Settings: {:#?}", self.settings).as_str(),
            5.,
            35.,
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
        x: f32,
        y: f32,
        transparency: f32,
        layer: Option<usize>,
    ) -> Result<(), RuntimeError> {
        if let Ok(animation) = self.resources.get_animation(id.to_string().as_str()) {
            return self
                .draw_grh(&animation.frames[0], x, y, transparency, layer)
                .await;
        } else {
            println!("animation not found {id}");
        }

        Err(RuntimeError::AnimationNotFound)
    }

    pub async fn draw_grh(
        &self,
        id: &str,
        x: f32,
        y: f32,
        transparency: f32,
        layer: Option<usize>,
    ) -> Result<(), RuntimeError> {
        if let Ok(image) = self.resources.get_image(id) {
            return self.draw_image(image, x, y, transparency, layer).await;
        }

        Err(RuntimeError::GrhNotFound)
    }

    pub async fn draw_image(
        &self,
        image: &Image,
        x: f32,
        y: f32,
        transparency: f32,
        layer: Option<usize>,
    ) -> Result<(), RuntimeError> {
        if layer.is_some() && self.settings.use_atlases {
            // let layer = layer.unwrap();

            let atlas_region = self
                .resources
                .get_map_atlas_region(1, image.file_num.to_string());
            if let Some(atlas_region) = atlas_region {
                let texture = self.resources.get_map_atlas_texture(1).await?;

                let x = x - (image.width / 2) as f32;
                let y = y - (image.height) as f32;

                let (texture_x, texture_y) =
                    self.resources.get_image_atlas_coords(atlas_region, image);

                let mut color = WHITE;
                color.a = transparency;

                draw_texture_ex(
                    texture,
                    x,
                    y,
                    color,
                    DrawTextureParams {
                        source: Some(Rect::new(
                            texture_x,
                            texture_y,
                            image.width as f32,
                            image.height as f32,
                        )),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            } else {
                return Err(RuntimeError::GrhNotFound);
            }
        } else {
            let texture = self.resources.get_texture(image.file_num as usize).await?;
            let x = x - (image.width / 2) as f32;
            let y = y - (image.height) as f32;
            let mut color = WHITE;
            color.a = transparency;

            draw_texture_ex(
                texture,
                x,
                y,
                color,
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
        }

        Ok(())
    }
}
