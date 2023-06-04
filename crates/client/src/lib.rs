use definitions::{
    atlas::{AtlasResource, AtlasType},
    client::ClientResources,
    client::ClientResourcesPaths,
    image::Image,
    Offset,
};
use itertools::iproduct;
use roma::{
    draw::{DrawParams, Rect},
    roma::{Game, Roma},
};
use std::{cmp::min, time::Duration};

use definitions::{client::load_client_resources, map::Map};
use entity::Entity;
use settings::Settings;

pub mod entity;
pub mod input;
pub mod settings;

pub struct Finisterra {
    pub settings: Settings,
    pub resources: ClientResources,
    pub current_map: Map,
    pub position: (usize, usize),
    pub entities: Vec<Entity>,
}

impl Default for Finisterra {
    fn default() -> Self {
        let atlas = AtlasResource {
            metadata_path: "./assets/finisterra/atlas.toml",
            image_id: 0,
            atlas_type: AtlasType::Yatp,
        };
        let paths = ClientResourcesPaths {
            bodies: "./assets/99z/Personajes.ind",
            heads: "./assets/99z/Cabezas.ind",
            weapons: "./assets/99z/Armas.dat",
            shields: "./assets/99z/Escudos.dat",
            headgears: "./assets/99z/Cascos.ind",
            fxs: "./assets/99z/Fxs.ind",
            maps: "./assets/99z/maps/",
            graphics: "./assets/99z/Graficos.ind",
            atlas: Some(atlas),
        };
        let resources = load_client_resources(paths).expect("can load client resources");
        let mut current_map = resources.maps.get(&1).expect("can get map").clone();
        let mut entities = vec![];
        for i in 0..10000 {
            let entity = Entity::random(1000000 + i * 10, &resources);
            current_map.tiles[entity.position[0]][99 - entity.position[1]].user = Some(i);
            println!("entity: {:?}", entity);
            entities.push(entity);
        }
        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (50, 50),
            entities,
        }
    }
}

impl Game for Finisterra {
    fn update(&mut self, roma: &mut Roma, delta: Duration) {
        self.process_input(roma, delta);
        self.update_camera(roma);
        self.draw_map(roma);
    }
}

const TILE_SIZE: usize = 32;

impl Finisterra {
    pub fn update_camera(&self, roma: &mut Roma) {
        let x = self.position.0 * 32 - 16;
        let y = self.position.1 * 32;
        roma.set_camera_position(x, y);
    }

    pub fn draw_map(&self, roma: &mut Roma) {
        let (x, y) = self.position;

        self.draw_layer_1(roma, y, x);
        self.draw_layer_2(roma, y, x);
        self.draw_layer_3(roma, y, x);
        self.draw_layer_0(roma, y, x);
    }

    fn draw_layer_0(&self, roma: &mut Roma, y: usize, x: usize) {
        let layer = 0;
        let range = range_by_layer(layer);
        let (y_start, y_end) = (y.saturating_sub(range), min(y + range, 99));
        let (x_start, x_end) = (x.saturating_sub(range), min(x + range, 99));

        for (y, x) in iproduct!(y_start..=y_end, x_start..=x_end) {
            let tile = &self.current_map.tiles[x][99 - y];
            if tile.graphics[layer] != 0 {
                let z = calculate_z(layer, y, x);
                let x = x * TILE_SIZE;
                let y = y * TILE_SIZE;
                let image_id = tile.graphics[layer] as usize;

                self.draw_grh(roma, image_id, x, y, z);
            }
        }
    }

    fn draw_layer_1(&self, roma: &mut Roma, y: usize, x: usize) {
        let layer = 1;
        let range = range_by_layer(layer);

        let (y_start, y_end) = (y.saturating_sub(range), min(y + range, 99));
        let (x_start, x_end) = (x.saturating_sub(range), min(x + range, 99));

        for (y, x) in iproduct!(y_start..=y_end, x_start..=x_end) {
            let tile = &self.current_map.tiles[x][99 - y];
            if tile.graphics[layer] != 0 {
                let z = calculate_z(layer, y, x);
                let x = x * TILE_SIZE;
                let y = y * TILE_SIZE;
                let image_id = tile.graphics[layer] as usize;

                self.draw_grh(roma, image_id, x, y, z);
            }
        }
    }

    fn draw_layer_2(&self, roma: &mut Roma, y: usize, x: usize) {
        let layer = 2;
        let range = range_by_layer(layer);

        let (y_start, y_end) = (y.saturating_sub(range), min(y + range, 99));
        let (x_start, x_end) = (x.saturating_sub(range), min(x + range, 99));

        for (y, x) in iproduct!(y_start..=y_end, x_start..=x_end) {
            let tile = &self.current_map.tiles[x][99 - y];
            if let Some(entity_id) = tile.user {
                let entity = &self.entities[entity_id];
                self.draw_entity(roma, entity, layer);
                continue;
            }
            if tile.graphics[layer] != 0 {
                let z = calculate_z(layer, y, x);
                let world_y = y * TILE_SIZE;
                let world_x = x * TILE_SIZE;
                let image_id = tile.graphics[layer] as usize;
                self.draw_grh(roma, image_id, world_x, world_y, z);
            }
        }
    }

    fn draw_layer_3(&self, roma: &mut Roma, y: usize, x: usize) {
        let layer = 3;
        let range = range_by_layer(layer);
        let (y_start, y_end) = (y.saturating_sub(range), min(y + range, 99));
        let (x_start, x_end) = (x.saturating_sub(range), min(x + range, 99));

        for (y, x) in iproduct!(y_start..=y_end, x_start..=x_end) {
            let tile = &self.current_map.tiles[x][99 - y];
            if tile.graphics[layer] != 0 {
                let z = calculate_z(layer, y, x);
                let x = x * TILE_SIZE;
                let y = y * TILE_SIZE;
                let image_id = tile.graphics[layer] as usize;

                self.draw_grh(roma, image_id, x, y, z);
            }
        }
    }

    const ZERO_OFFSET: &Offset = &Offset { x: 0, y: 0 };
    fn draw_entity(&self, roma: &mut Roma, entity: &Entity, layer: usize) {
        let x = entity.position[0];
        let y = entity.position[0];
        let z = calculate_z(layer, y, x);
        let world_x = entity.world_position[0];
        let world_y = entity.world_position[1];

        if entity.body != 0 {
            let y = world_y - 20;
            let head_offset = if let Some(body) = self.resources.bodies.get(&entity.body) {
                self.draw_animation(roma, body.animations[0], world_x, y, z);
                &body.head_offset
            } else {
                Self::ZERO_OFFSET
            };
            if entity.head != 0 {
                if let Some(head) = self.resources.heads.get(&entity.head) {
                    let x = world_x - head_offset.x;
                    let y = y - head_offset.y;
                    self.draw_grh(roma, head.images[0], x, y, z);
                }
            }
        }
    }

    fn draw_animation(&self, roma: &mut Roma, id: usize, x: usize, y: usize, z: f32) {
        if let Some(animation) = self.resources.animations.get(&id) {
            self.draw_grh(roma, animation.frames[0], x, y, z);
        }
    }

    fn draw_grh(&self, roma: &mut Roma, image_id: usize, x: usize, y: usize, z: f32) {
        if let Some(image) = self.resources.images.get(&image_id) {
            self.draw_image(roma, image, x, y, z);
        } else {
            println!("> draw_grh > image not found: {}", image_id);
        }
    }

    fn draw_image(&self, roma: &mut Roma, image: &Image, x: usize, y: usize, z: f32) {
        let image_num = image.file_num as usize;
        let x = (x as f32 - (image.width as f32 / 2.)).round() as usize;

        roma.draw_texture(DrawParams {
            texture_id: image_num,
            x,
            y,
            z,
            source: Some(Rect::new(
                image.x.into(),
                image.y.into(),
                image.width.into(),
                image.height.into(),
            )),
            flip_y: true,
        });
    }
}

fn calculate_z(layer: usize, y: usize, x: usize) -> f32 {
    (match layer {
        0 => 0,
        3 => 4000,
        _ => layer * 1000 + (100 - y) * 10 + x,
    }) as f32
        / 4000.
}

fn range_by_layer(layer: usize) -> usize {
    match layer {
        0 => 8,
        1 => 8,
        2 => 10,
        _ => 9,
    }
}
