use definitions::{
    animation::Animation,
    ao_20, ao_99z,
    atlas::{AtlasResource, AtlasType},
    client::ClientResources,
    image::Image,
};
use itertools::iproduct;
use roma::*;
use std::cmp::min;
use ui::UI;
use z_ordering::{get_shield_z, get_weapon_z};

use definitions::map::Map;
use entity::Entity;
use settings::Settings;

pub mod entity;
pub mod input;
pub mod settings;
pub mod ui;

mod z_ordering;

pub const WINDOW_WIDTH: usize = 1920;
pub const WINDOW_HEIGHT: usize = 1080;

pub const TILES: usize = 15;

const TILE_SIZE: usize = 32;
const HALF_TILE: usize = TILE_SIZE / 2;

const LAYERS: usize = 4;
const MAP_WIDTH: usize = 100;
const MAP_HEIGHT: usize = 100;

lazy_static::lazy_static! {
    static ref Z_ORDERING: Vec<Vec<Vec<f32>>> = {
        let mut z_ordering = vec![vec![vec![0.; MAP_HEIGHT]; MAP_WIDTH]; LAYERS];

        for (layer, layer_ordering) in z_ordering.iter_mut().enumerate().take(LAYERS) {
            for (x, x_vec) in layer_ordering.iter_mut().enumerate().take(MAP_WIDTH) {
                for (y, z) in x_vec.iter_mut().enumerate().take(MAP_HEIGHT) {
                   *z = calculate_z(layer, x, y);
                }
            }

        }
        z_ordering
    };
}

pub struct Finisterra {
    pub settings: Settings,
    pub resources: ClientResources,
    pub current_map: Map,
    pub position: (f32, f32),
    pub entities: Vec<Entity>,

    pub ui: UI,

    pub window_size: (usize, usize),
    pub render_size: (usize, usize),
}

impl Finisterra {
    pub fn ao_20(ui: UI) -> Self {
        let paths = ao_20::client::ClientResourcesPaths {
            bodies: "./assets/ao_20/init/cuerpos.dat",
            templates: "./assets/ao_20/init/moldes.ini",
            heads: "./assets/ao_20/init/cabezas.ini",
            weapons: "./assets/ao_20/init/armas.dat",
            shields: "./assets/ao_20/init/escudos.dat",
            headgears: "./assets/ao_20/init/cascos.ini",
            fxs: "./assets/ao_20/init/fxs.ind",
            maps: "./assets/ao_20/maps/",
            graphics: "./assets/ao_20/init/graficos.ind",
            atlas: None,
        };
        let resources =
            ao_20::client::load_client_resources(paths).expect("can load client resources");
        let current_map = resources.maps.get(&1).expect("can get map").clone();

        let entities = vec![];

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (0., 0.),
            entities,
            ui,
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
        }
    }

    pub fn ao_99z(ui: UI) -> Self {
        let atlas = AtlasResource {
            metadata_path: "./assets/finisterra/atlas.toml",
            image_id: 0,
            atlas_type: AtlasType::Yatp,
        };
        let paths = ao_99z::client::ClientResourcesPaths {
            bodies: "./assets/ao_99z/Personajes.ind",
            heads: "./assets/ao_99z/Cabezas.ind",
            weapons: "./assets/ao_99z/Armas.dat",
            shields: "./assets/ao_99z/Escudos.dat",
            headgears: "./assets/ao_99z/Cascos.ind",
            fxs: "./assets/ao_99z/Fxs.ind",
            maps: "./assets/ao_99z/maps/",
            graphics: "./assets/ao_99z/Graficos.ind",
            atlas: Some(atlas),
        };

        let resources =
            ao_99z::client::load_client_resources(paths).expect("can load client resources");

        let current_map = resources.maps.get(&1).expect("can get map").clone();
        let entities = vec![];

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (50., 50.),
            entities,
            ui,
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
        }
    }

    pub fn game_loop(&mut self) {
        self.process_input();
        self.update_camera();
        self.update_entities();
        self.draw_map();
        self.draw_ui();
    }

    pub fn resize(&mut self, window_size: (usize, usize)) {
        self.ui.resize(window_size);
        println!("new window size {}-{}", window_size.0, window_size.1);
        let render_width = window_size.0 - self.ui.border * 2 - self.ui.right_panel_size;
        let render_height = window_size.1 - self.ui.border * 2 - self.ui.top_panel_size;

        self.render_size = (render_width, render_height);
        self.window_size = window_size;

        // set_camera_size(self.render_size.0 as f32, self.render_size.1 as f32);
        set_camera_size(
            self.window_size.0 as f32 - 40.,
            self.window_size.1 as f32 - 40.,
        );

        if render_width > TILES * TILE_SIZE * 2 {
            set_camera_zoom(Zoom::Double);
        } else {
            set_camera_zoom(Zoom::None);
        }
    }

    fn get_render_tiles(&self, extra: usize) -> (usize, usize) {
        let (render_width, render_height) = self.render_size;
        let zoom = match get_camera_zoom() {
            Zoom::None => 1.,
            Zoom::Double => 2.,
        };

        (
            (render_width as f32 / TILE_SIZE as f32 / zoom).ceil() as usize + extra,
            (render_height as f32 / TILE_SIZE as f32 / zoom).ceil() as usize + extra,
        )
    }

    fn update_entities(&mut self) {
        let delta = get_delta();

        let (x, y) = (self.position.0 as usize, self.position.1 as usize);
        let (w_range, h_range) = self.get_render_tiles(0);
        let (y_start, y_end) = (y.saturating_sub(h_range), min(y + h_range, MAP_HEIGHT));
        let (x_start, x_end) = (x.saturating_sub(w_range), min(x + w_range, MAP_WIDTH));
        for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
            let tile = &self.current_map.tiles[x][y];
            if let Some(user) = tile.user {
                let entity = &mut self.entities[user];
                entity.update(delta);
            }
        }
    }

    fn update_camera(&self) {
        let x = self.position.0 * TILE_SIZE as f32 - HALF_TILE as f32;
        let y = self.position.1 * TILE_SIZE as f32;
        set_camera_position(x.floor(), y.floor());
    }

    fn draw_map(&self) {
        let (x, y) = (self.position.0 as usize, self.position.1 as usize);

        let (w_range, h_range) = self.get_render_tiles(3);
        let (y_start, y_end) = (y.saturating_sub(h_range), min(y + h_range, MAP_HEIGHT));
        let (x_start, x_end) = (x.saturating_sub(w_range), min(x + w_range, MAP_WIDTH));
        for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
            let tile = &self.current_map.tiles[x][y];
            let world_x = (x * TILE_SIZE) as f32;
            let world_y = ((y * TILE_SIZE) - TILE_SIZE) as f32;

            for layer in 0..LAYERS {
                if tile.graphics[layer] != 0 {
                    let z = Z_ORDERING[layer][x][y];
                    self.draw_grh(tile.graphics[layer] as u32, world_x, world_y, z);
                }
            }

            if let Some(user) = tile.user {
                let entity = &self.entities[user];
                self.draw_entity(entity, 2);
            }
        }
    }

    fn draw_entity(&self, entity: &Entity, layer: usize) {
        let [x, y] = entity.position;
        let [world_x, world_y] = entity.world_position;
        let z = Z_ORDERING[layer][x as usize][y as usize];

        if let Some((body, (animations, frame))) = &entity.body {
            self.draw_animation(
                &animations[entity.state.direction as usize],
                *frame,
                world_x,
                world_y,
                z,
            );
            let head_offset = &body.head_offset;
            if let Some((_, images)) = &entity.head {
                let x = (world_x as isize - head_offset.x) as f32;
                let y = (world_y as isize - head_offset.y) as f32;
                let image = &images[entity.state.direction as usize];
                self.draw_image(image, x, y, z);
            }
            if let Some((_, (animations, frame))) = &entity.weapon {
                self.draw_animation(
                    &animations[entity.state.direction as usize],
                    *frame,
                    world_x,
                    world_y,
                    get_weapon_z(z, entity.state.direction),
                );
            }
            if let Some((_, (animations, frame))) = &entity.shield {
                self.draw_animation(
                    &animations[entity.state.direction as usize],
                    *frame,
                    world_x,
                    world_y,
                    get_shield_z(z, entity.state.direction),
                );
            }
        }

        let position = [world_x, world_y - 10., z];
        let color = [0, 128, 255, 255];
        draw_parsed_text(&entity.name.1, &position, color);
    }

    fn draw_animation(&self, animation: &Animation, frame: usize, x: f32, y: f32, z: f32) {
        if animation.frames.is_empty() {
            return;
        }
        self.draw_grh(animation.frames[frame], x, y, z);
    }

    fn draw_grh(&self, image_id: u32, x: f32, y: f32, z: f32) {
        if let Some(image) = &self.resources.images[image_id as usize] {
            self.draw_image(image, x, y, z);
        }
    }

    fn draw_image(&self, image: &Image, x: f32, y: f32, z: f32) {
        let image_num = image.file_num;
        let x = x - (image.width as f32 / 2.);

        draw_image(
            image_num,
            DrawImageParams::new(
                &[x, y, z],
                [255, 255, 255, 255],
                [image.x, image.y, image.width, image.height],
            ),
        );
    }

    fn draw_ui(&mut self) {
        self.ui.draw(self.window_size, self.render_size);
    }
}

fn calculate_z(layer: usize, x: usize, y: usize) -> f32 {
    match layer {
        0 => 0.,
        3 => 1.,
        _ => (2000. + (100. - y as f32) * 10. - x as f32) / 4000.,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_z() {
        let z_1 = dbg!(calculate_z(2, 0, 0));
        let z_2 = dbg!(calculate_z(2, 0, 1));
        let z_3 = dbg!(calculate_z(2, 1, 0));
        let z_4 = dbg!(calculate_z(2, 1, 1));
        // let z_5 = dbg!(calculate_z(2, 51., 51.));
        assert!(z_1 > z_2);
        assert!(z_1 > z_3);
        assert!(z_2 > z_4);
        assert!(z_3 > z_2);
    }
}
