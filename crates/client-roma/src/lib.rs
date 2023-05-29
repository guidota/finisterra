use std::{cmp::min, time::Duration};

use ao::{
    ao_20::{
        graphics::Image,
        init::{parse::template::Template, Body},
        maps::parse::parse_map,
    },
    Map,
};
use entity::Entity;
use resources::Resources;
use roma::{
    graphics::{
        rect::Rect,
        textures::DrawTextureParams,
        vec2::{vec2, Vec2},
    },
    Color, Game, Roma,
};
use settings::Settings;

pub mod entity;
pub mod input;
pub mod resources;
pub mod settings;

pub struct Finisterra {
    pub settings: Settings,
    pub resources: Resources,
    pub current_map: Map,
    pub position: Vec2,
    pub entities: Vec<Entity>,
}

impl Default for Finisterra {
    fn default() -> Self {
        let current_map = parse_map("./assets/maps", 1).expect("can parse map");
        let resources = Resources::load();
        let mut entities = vec![];
        for _ in 1..=10000 {
            entities.push(Entity::random(&resources));
        }
        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: vec2(50., 50.),
            entities,
        }
    }
}

impl Game for Finisterra {
    fn update(&mut self, roma: &mut Roma, delta: Duration) {
        self.process_input(roma, delta);
        self.update_camera(roma);
        self.draw_map(roma);
        self.draw_entities(roma);
    }
}

const TILE_SIZE: usize = 32;

impl Finisterra {
    pub fn update_camera(&self, roma: &mut Roma) {
        let camera_position = self.position * 32.;
        roma.set_camera_position(camera_position);
    }

    pub fn draw_map(&self, roma: &mut Roma) {
        let (x, y) = (self.position.x as usize, self.position.y as usize);

        for layer in 0..4 {
            let range = range_by_layer(layer);
            let (y_start, y_end) = (y.saturating_sub(range), min(y + range, 100));
            let (x_start, x_end) = (x.saturating_sub(range), min(x + range, 100));

            for y in y_start..y_end {
                for x in x_start..x_end {
                    let tile = self.current_map.tiles[x][99 - y];
                    if tile.graphics[layer] != 0 {
                        let z = calculate_z(layer, y, x);
                        let x = (x * TILE_SIZE) as f32;
                        let y = (y * TILE_SIZE) as f32;
                        let image_id = tile.graphics[layer] as usize;
                        self.draw_grh(roma, image_id, x, y, z);
                    }
                }
            }
        }
    }
    pub fn draw_entities(&self, roma: &mut Roma) {
        let entities = self
            .entities
            .iter()
            .filter(|e| self.position.distance(e.position) < 15.);
        for entity in entities {
            let Vec2 { x, y } = entity.position;
            let z = calculate_z(2, y as usize, x as usize);
            let x = x * 32.;
            let y = y * 32.;

            if entity.body != 0 {
                let y = y - 20.;
                let head_offset = match self.resources.bodies.get(&entity.body) {
                    Some(Body::Animated { walks, head_offset }) => {
                        let body_grh = walks.0;
                        self.draw_animation(roma, body_grh, x, y, z);
                        head_offset
                    }
                    Some(Body::AnimatedWithTemplate {
                        template_id,
                        file_num,
                        head_offset,
                    }) => {
                        if let Some(template) = self.resources.body_templates.get(template_id) {
                            self.draw_template(roma, file_num, template, x, y, z);
                        }
                        head_offset
                    }
                    None => &(0, 0),
                };
                if entity.head != 0 {
                    if let Some(head) = self.resources.heads.get(&entity.head) {
                        let x = x - head_offset.0 as f32;
                        let y = y - head_offset.1 as f32;
                        self.draw_grh(roma, head.2, x, y, z);
                    }
                }
            }
        }
    }

    fn draw_template(
        &self,
        roma: &mut Roma,
        file_num: &usize,
        template: &Template,
        x: f32,
        y: f32,
        z: usize,
    ) {
        let id = String::new();
        let image = ao::ao_20::graphics::Image {
            file_num: *file_num as u32,
            x: template.x as u16,
            y: template.y as u16,
            width: template.width as u16,
            height: template.height as u16,
            id,
        };
        self.draw_image(roma, &image, x, y, z);
    }

    fn draw_animation(&self, roma: &mut Roma, id: usize, x: f32, y: f32, z: usize) {
        if let Some(animation) = self.resources.animations.get(id.to_string().as_str()) {
            self.draw_grh(roma, animation.frames[0].parse().unwrap(), x, y, z);
        }
    }

    fn draw_grh(&self, roma: &mut Roma, image_id: usize, x: f32, y: f32, z: usize) {
        if let Some(image) = self.resources.images.get(&image_id.to_string()) {
            self.draw_image(roma, image, x, y, z);
        }
    }

    fn draw_image(&self, roma: &mut Roma, image: &Image, x: f32, y: f32, z: usize) {
        let texture_id = image.file_num.to_string();
        let image_path = format!("./assets/graphics/{texture_id}.png");
        roma.graphics.load_texture(texture_id.clone(), &image_path);

        let x = x - (image.width / 2) as f32;

        roma.graphics.draw_texture(
            texture_id,
            x,
            y,
            z,
            Color::WHITE,
            Some(DrawTextureParams {
                source: Some(Rect::new(
                    image.x as f32,
                    image.y as f32,
                    image.width as f32,
                    image.height as f32,
                )),
                flip_y: true,
            }),
        );
    }
}

fn calculate_z(layer: usize, y: usize, x: usize) -> usize {
    match layer {
        0 => 0,
        3 => 4000,
        _ => layer * 1000 + (100 - y) * 10 + x,
    }
}

fn range_by_layer(layer: usize) -> usize {
    match layer {
        0 => 15,
        1 => 18,
        2 => 16,
        _ => 20,
    }
}
