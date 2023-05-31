use definitions::{client::ClientResources, client::ClientResourcesPaths, image::Image, Offset};
use std::{cmp::min, time::Duration};

use definitions::{client::load_client_resources, map::Map};
use entity::Entity;
use roma::{
    graphics::{
        rect::Rect,
        textures::DrawTextureParams,
        vec2::{vec2, Vec2},
    },
    Game, Roma,
};
use settings::Settings;

pub mod entity;
pub mod input;
pub mod settings;

pub struct Finisterra {
    pub settings: Settings,
    pub resources: ClientResources,
    pub current_map: Map,
    pub position: Vec2,
    pub entities: Vec<Entity>,
}

impl Default for Finisterra {
    fn default() -> Self {
        let paths = ClientResourcesPaths {
            bodies: "",
            heads: "",
            weapons: "",
            shields: "",
            headgears: "",
            fxs: "",
            maps: "",
            graphics: "",
        };
        let resources = load_client_resources(paths).expect("can load client resources");
        let current_map = resources.maps.get(&1).expect("can get map").clone();
        let mut entities = vec![];
        for i in 1..=10000 {
            entities.push(Entity::random(1000000 + i * 10, &resources));
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
                    let tile = &self.current_map.tiles[x][99 - y];
                    if tile.graphics[layer] != 0 {
                        let z = calculate_z(layer, y, x);
                        let x = (x * TILE_SIZE) as f32;
                        let y = (y * TILE_SIZE) as f32;
                        let image_id = tile.graphics[layer] as usize;
                        let tile_id = y as usize * 10000 + x as usize * 4 + layer;
                        self.draw_grh(roma, tile_id, image_id, x, y, z);
                    }
                }
            }
        }
    }

    const ZERO_OFFSET: &Offset = &Offset { x: 0, y: 0 };

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
                let head_offset = if let Some(body) = self.resources.bodies.get(&entity.body) {
                    self.draw_animation(roma, entity.id + 1, body.animations[0], x, y, z);
                    &body.head_offset
                } else {
                    Self::ZERO_OFFSET
                };
                if entity.head != 0 {
                    if let Some(head) = self.resources.heads.get(&entity.head) {
                        let x = x - head_offset.x as f32;
                        let y = y - head_offset.y as f32;
                        self.draw_grh(roma, entity.id + 3, head.images[0], x, y, z);
                    }
                }
            }
        }
    }

    fn draw_animation(
        &self,
        roma: &mut Roma,
        entity_id: usize,
        id: usize,
        x: f32,
        y: f32,
        z: usize,
    ) {
        if let Some(animation) = self.resources.animations.get(&id) {
            self.draw_grh(roma, entity_id, animation.frames[0], x, y, z);
        }
    }

    fn draw_grh(
        &self,
        roma: &mut Roma,
        entity_id: usize,
        image_id: usize,
        x: f32,
        y: f32,
        z: usize,
    ) {
        if let Some(image) = self.resources.images.get(&image_id) {
            self.draw_image(roma, entity_id, image, x, y, z);
        }
    }

    fn draw_image(
        &self,
        roma: &mut Roma,
        entity_id: usize,
        image: &Image,
        x: f32,
        y: f32,
        z: usize,
    ) {
        let texture_id = image.file_num.to_string();
        let image_path = format!("./assets/graphics/{texture_id}.png");
        roma.graphics.load_texture(texture_id.clone(), &image_path);

        let x = x - (image.width / 2) as f32;

        roma.graphics.draw_texture(
            entity_id,
            texture_id,
            x,
            y,
            z,
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
