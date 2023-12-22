use definitions::{
    animation::Animation,
    ao_20, ao_99z,
    atlas::{AtlasResource, AtlasType},
    client::ClientResources,
    image::Image,
};
use engine::{
    camera::Zoom,
    draw::{
        image::DrawImage,
        text::{DrawText, Orientation},
        Position,
    },
    engine::GameEngine,
    game::Game,
};
use itertools::iproduct;
use std::cmp::min;
use z_ordering::{get_headgear_z, get_shield_z, get_weapon_z};

use definitions::map::Map;
use entity::Entity;
use settings::Settings;

pub mod entity;
pub mod input;
pub mod settings;

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

    // pub ui: UI,
    pub window_size: (usize, usize),
    pub render_size: (usize, usize),

    pub render_names: bool,
}

impl Game for Finisterra {
    fn initialize<E: engine::engine::GameEngine>(engine: &mut E) -> Self {
        let mut finisterra = Self::ao_20(engine);

        const CHARS: usize = 100;
        for i in 0..CHARS {
            let entity = Entity::random(1000000 + i * 10, &finisterra.resources);

            let [x, y] = entity.position;
            finisterra.current_map.tiles[x as usize][y as usize].user = Some(i);
            finisterra.entities.push(entity);
        }

        finisterra
    }

    fn tick<E: engine::engine::GameEngine>(&mut self, engine: &mut E) {
        self.game_loop(engine)
    }
}

impl Finisterra {
    pub const TAHOMA_ID: u64 = 1;

    pub fn ao_20<E: engine::engine::GameEngine>(engine: &mut E) -> Self {
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

        // register textures
        for image in resources.images.iter().flatten() {
            engine.set_texture(
                format!("./assets/ao_20/graphics/{}.png", image.file_num).as_str(),
                image.file_num,
            );
        }

        let texture_id = engine.add_texture("./assets/fonts/shadowed-font.png");
        engine.add_font(Self::TAHOMA_ID, "./assets/fonts/font.fnt", texture_id);

        let entities = vec![];

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (0., 0.),
            entities,
            // ui,
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_size: (WINDOW_WIDTH, WINDOW_HEIGHT),

            render_names: true,
        }
    }

    pub fn ao_99z<E: engine::engine::GameEngine>(engine: &mut E) -> Self {
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

        // register textures
        for image in resources.images.iter().flatten() {
            engine.set_texture(
                format!("./assets/ao_99z/graphics/{}.png", image.file_num).as_str(),
                image.file_num,
            );
        }

        let texture_id = engine.add_texture("./assets/fonts/shadowed-font.png");
        engine.add_font(Self::TAHOMA_ID, "./assets/fonts/font.fnt", texture_id);

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (50., 50.),
            entities,
            // ui,
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_names: true,
        }
    }

    pub fn game_loop<E: GameEngine>(&mut self, engine: &mut E) {
        self.process_input(engine);
        self.update_camera(engine);
        self.update_entities(engine);
        self.draw_map(engine);
        self.draw_ui(engine);
    }
    //
    // pub fn resize(&mut self, window_size: (usize, usize)) {
    //     // self.ui.resize(window_size);
    //     println!("new window size {}-{}", window_size.0, window_size.1);
    //     let render_width = window_size.0 - self.ui.border * 2 - self.ui.right_panel_size;
    //     let render_height = window_size.1 - self.ui.border * 2 - self.ui.top_panel_size;
    //
    //     self.render_size = (render_width, render_height);
    //     self.window_size = window_size;
    //
    //     set_camera_size(self.render_size.0 as f32, self.render_size.1 as f32);
    //     // set_camera_size(
    //     //     self.window_size.0 as f32 - 40.,
    //     //     self.window_size.1 as f32 - 40.,
    //     // );
    //
    //     if render_width > TILES * TILE_SIZE * 2 {
    //         set_camera_zoom(Zoom::Double);
    //     } else {
    //         set_camera_zoom(Zoom::None);
    //     }
    // }

    fn get_render_tiles(&self, extra: usize, zoom: Zoom) -> (usize, usize) {
        let (render_width, render_height) = self.render_size;
        let zoom = match zoom {
            Zoom::None => 1.,
            Zoom::Double => 2.,
        };

        (
            (render_width as f32 / TILE_SIZE as f32 / zoom).ceil() as usize + extra,
            (render_height as f32 / TILE_SIZE as f32 / zoom).ceil() as usize + extra,
        )
    }

    fn update_entities<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();

        let (x, y) = (self.position.0 as usize, self.position.1 as usize);
        let (w_range, h_range) = self.get_render_tiles(0, engine.get_world_camera_zoom());
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

    fn update_camera<E: GameEngine>(&mut self, engine: &mut E) {
        let window_size = engine.get_window_size();
        let height = 32. * 14. * 2.;
        let width = 32. * 19. * 2.;
        engine.set_world_camera_viewport(engine::camera::Viewport {
            x: 10.,
            y: window_size.height as f32 - 10. - height,
            width,
            height,
        });
        engine.set_ui_camera_viewport(engine::camera::Viewport {
            x: 0.,
            y: 0.,
            width: window_size.width as f32,
            height: window_size.height as f32,
        });
        let x = self.position.0 * TILE_SIZE as f32 - HALF_TILE as f32;
        let y = self.position.1 * TILE_SIZE as f32;
        engine.set_world_camera_position(engine::camera::Position {
            x: x.floor(),
            y: y.floor(),
        });
    }

    fn draw_map<E: GameEngine>(&mut self, engine: &mut E) {
        let (x, y) = (self.position.0 as usize, self.position.1 as usize);

        let (w_range, h_range) = self.get_render_tiles(3, engine.get_world_camera_zoom());
        let (y_start, y_end) = (y.saturating_sub(h_range), min(y + h_range, MAP_HEIGHT));
        let (x_start, x_end) = (x.saturating_sub(w_range), min(x + w_range, MAP_WIDTH));
        for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
            let tile = &self.current_map.tiles[x][y];
            let world_x = (x * TILE_SIZE) as f32;
            let world_y = ((y * TILE_SIZE) - TILE_SIZE) as f32;

            for layer in 0..LAYERS {
                if tile.graphics[layer] != 0 {
                    let z = Z_ORDERING[layer][x][y];
                    self.draw_grh(engine, tile.graphics[layer] as u32, world_x, world_y, z);
                }
            }

            if let Some(user) = tile.user {
                let entity = &self.entities[user];
                self.draw_entity(engine, entity, 2);
            }
        }
    }

    fn draw_entity<E: GameEngine>(&self, engine: &mut E, entity: &Entity, layer: usize) {
        let [x, y] = entity.position;
        let [world_x, world_y] = entity.world_position;
        let z = Z_ORDERING[layer][x as usize][y as usize];

        if let Some((body, (animations, frame))) = &entity.body {
            self.draw_animation(
                engine,
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
                self.draw_image(engine, image, x, y, z);
            }

            if let Some((_, images)) = &entity.head_gear {
                let x = (world_x as isize - head_offset.x) as f32;
                let y = (world_y as isize - head_offset.y) as f32;
                let image = &images[entity.state.direction as usize];
                self.draw_image(
                    engine,
                    image,
                    x,
                    y,
                    get_headgear_z(z, entity.state.direction),
                );
            }

            if let Some((_, (animations, frame))) = &entity.weapon {
                self.draw_animation(
                    engine,
                    &animations[entity.state.direction as usize],
                    *frame,
                    world_x,
                    world_y,
                    get_weapon_z(z, entity.state.direction),
                );
            }
            if let Some((_, (animations, frame))) = &entity.shield {
                self.draw_animation(
                    engine,
                    &animations[entity.state.direction as usize],
                    *frame,
                    world_x,
                    world_y,
                    get_shield_z(z, entity.state.direction),
                );
            }
        }

        if self.render_names {
            let color = [0, 128, 255, 255];
            engine.draw_text(
                Self::TAHOMA_ID,
                DrawText {
                    text: &entity.name,
                    position: Position {
                        x: world_x as u16,
                        y: world_y as u16 - 10,
                        z,
                        ui: false,
                    },
                    color,
                    orientation: Orientation::Center,
                },
            );
        }
    }

    fn draw_animation<E: GameEngine>(
        &self,
        engine: &mut E,
        animation: &Animation,
        frame: usize,
        x: f32,
        y: f32,
        z: f32,
    ) {
        if animation.frames.is_empty() {
            return;
        }
        self.draw_grh(engine, animation.frames[frame], x, y, z);
    }

    fn draw_grh<E: GameEngine>(&self, engine: &mut E, image_id: u32, x: f32, y: f32, z: f32) {
        if let Some(image) = &self.resources.images[image_id as usize] {
            self.draw_image(engine, image, x, y, z);
        }
    }

    fn draw_image<E: GameEngine>(&self, engine: &mut E, image: &Image, x: f32, y: f32, z: f32) {
        let image_num = image.file_num;
        let x = x - (image.width as f32 / 2.);

        engine.draw_image(
            image_num,
            DrawImage {
                position: engine::draw::Position {
                    x: x as u16,
                    y: y as u16,
                    z,
                    ui: false,
                },
                color: [255, 255, 255, 255],
                source: [image.x, image.y, image.width, image.height],
            },
        );
    }

    fn draw_ui<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();

        engine.draw_text(
            Self::TAHOMA_ID,
            DrawText {
                text: &format!("FPS: {:.2}", 1. / delta.as_secs_f32()),
                position: Position {
                    x: 50,
                    y: 15,
                    z: 1.,
                    ui: true,
                },
                color: [255, 255, 255, 255],
                orientation: Orientation::Center,
            },
        );
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
