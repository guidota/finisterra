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
    pub position: (f32, f32),
    pub entities: Vec<Entity>,

    tiles_w: usize,
    tiles_h: usize,
}

pub const RENDER_W: usize = 480;
pub const RENDER_H: usize = 480;

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

        for i in 0..20000 {
            let entity = Entity::random(1000000 + i * 10, &resources);
            current_map.tiles[entity.position[0]][entity.position[1]].user = Some(i);
            println!("entity: {:?}", entity);
            entities.push(entity);
        }

        let tiles_w = ((RENDER_W as f32 / TILE_SIZE as f32).ceil() / 2.).ceil() as usize + 1;
        let tiles_h = ((RENDER_H as f32 / TILE_SIZE as f32).ceil() / 2.).ceil() as usize + 2;

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (50., 50.),
            entities,
            tiles_w,
            tiles_h,
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
const HALF_TILE: usize = TILE_SIZE / 2;

impl Finisterra {
    pub fn update_camera(&self, roma: &mut Roma) {
        let x = (self.position.0 * 32. - HALF_TILE as f32) as usize;
        let y = (self.position.1 * 32.) as usize;
        roma.set_camera_position(x, y);
    }

    pub fn draw_map(&self, roma: &mut Roma) {
        let (x, y) = (self.position.0 as usize, self.position.1 as usize);

        let (y_start, y_end) = (y.saturating_sub(self.tiles_h), min(y + self.tiles_h, 99));
        let (x_start, x_end) = (x.saturating_sub(self.tiles_w), min(x + self.tiles_w, 99));
        for (y, x) in iproduct!(y_start..=y_end, x_start..=x_end) {
            let tile = &self.current_map.tiles[x][y];
            for layer in 0..4 {
                if tile.graphics[layer] != 0 {
                    let z = calculate_z(layer, y, x);
                    let x = x * TILE_SIZE;
                    let y = y * TILE_SIZE;
                    let image_id = tile.graphics[layer] as usize;
                    self.draw_grh(roma, image_id, x, y, z);
                }
            }
            if let Some(user) = tile.user {
                let entity = &self.entities[user];
                self.draw_entity(roma, entity, 2);
            }
        }
    }

    const ZERO_OFFSET: &Offset = &Offset { x: 0, y: 0 };
    fn draw_entity(&self, roma: &mut Roma, entity: &Entity, layer: usize) {
        let x = entity.position[0];
        let y = entity.position[1];
        let z = calculate_z(layer, y, x);
        let world_x = entity.world_position[0];
        let world_y = entity.world_position[1];

        if entity.body != 0 {
            let y = world_y;
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
        }
    }

    fn draw_image(&self, roma: &mut Roma, image: &Image, x: usize, y: usize, z: f32) {
        let image_num = image.file_num;
        let x = x - image.width / 2;

        roma.draw_texture(DrawParams {
            texture_id: image_num,
            x,
            y,
            z,
            source: Some(Rect::new(image.x, image.y, image.width, image.height)),
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
