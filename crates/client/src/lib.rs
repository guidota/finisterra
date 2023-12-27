use definitions::{
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
        Color, Position, Target,
    },
    engine::GameEngine,
    game::Game,
};
use itertools::iproduct;
use std::cmp::min;

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
    pub static ref Z_ORDERING: Vec<Vec<Vec<f32>>> = {
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

#[derive(Debug)]
pub enum RenderTarget {
    Uninitialized,
    Dirty { texture_id: u64 },
    Ready { texture_id: u64 },
}

#[derive(Debug)]
pub enum TextureState {
    Dirty,
    Ready,
}

pub struct Finisterra {
    pub settings: Settings,
    pub resources: ClientResources,
    pub current_map: Map,
    pub position: (f32, f32),
    pub entities: Vec<Entity>,

    pub window_size: (usize, usize),
    pub render_size: (usize, usize),

    pub map_layer_textures: [RenderTarget; 4],

    pub draw_entities: bool,
    pub draw_names: bool,
    pub draw_map: bool,
}

impl Game for Finisterra {
    fn initialize<E: engine::engine::GameEngine>(engine: &mut E) -> Self {
        let mut finisterra = Self::ao_20(engine);

        const CHARS: usize = 1;
        for i in 0..CHARS {
            let entity = Entity::random(engine, i, &finisterra.resources);

            let [x, y] = entity.position;
            finisterra.current_map.tiles[x as usize][y as usize].user = Some(i);
            finisterra.entities.push(entity);
        }

        let position = &finisterra.entities[0].position.clone();
        finisterra.position = (position[0], position[1]);

        let mut i = finisterra.entities.len();
        let ratio = 1;
        for x in -ratio..=ratio {
            for y in -ratio..=ratio {
                if x == 0 && y == 0 {
                    continue;
                }
                let id = i;
                let mut entity = Entity::random(engine, id, &finisterra.resources);

                entity.set_position(position[0] + x as f32, position[1] + y as f32);

                finisterra.current_map.tiles[entity.position[0] as usize]
                    [entity.position[1] as usize]
                    .user = Some(id);
                finisterra.entities.push(entity);

                i += 1;
            }
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

        let dimensions = engine::draw::Dimensions {
            width: (TILE_SIZE * MAP_WIDTH) as u16,
            height: (TILE_SIZE * MAP_HEIGHT) as u16,
        };
        let map_layer_textures = [
            RenderTarget::Dirty {
                texture_id: engine.create_texture(dimensions),
            },
            RenderTarget::Dirty {
                texture_id: engine.create_texture(dimensions),
            },
            RenderTarget::Dirty {
                texture_id: engine.create_texture(dimensions),
            },
            RenderTarget::Dirty {
                texture_id: engine.create_texture(dimensions),
            },
        ];

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (0., 0.),
            entities,

            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_size: (WINDOW_WIDTH, WINDOW_HEIGHT),

            map_layer_textures,

            draw_entities: true,
            draw_names: true,
            draw_map: true,
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

        let dimensions = engine::draw::Dimensions {
            width: (TILE_SIZE * MAP_WIDTH) as u16,
            height: (TILE_SIZE * MAP_HEIGHT) as u16,
        };
        let map_layer_textures = [
            RenderTarget::Dirty {
                texture_id: engine.create_texture(dimensions),
            },
            RenderTarget::Uninitialized,
            RenderTarget::Uninitialized,
            RenderTarget::Dirty {
                texture_id: engine.create_texture(dimensions),
            },
        ];

        Self {
            settings: Settings::default(),
            resources,
            current_map,
            position: (50., 50.),
            entities,

            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            render_size: (WINDOW_WIDTH, WINDOW_HEIGHT),

            map_layer_textures,
            draw_entities: true,
            draw_names: true,
            draw_map: true,
        }
    }

    pub fn game_loop<E: GameEngine>(&mut self, engine: &mut E) {
        self.process_input(engine);
        self.update_camera(engine);
        self.update_entities(engine);
        self.draw_map(engine);
        self.draw_ui(engine);
    }

    fn get_render_tiles(&self, extra: usize, zoom: Zoom) -> (usize, usize) {
        let (render_width, render_height) = self.render_size;
        let zoom = match zoom {
            Zoom::None => 1.,
            Zoom::Double => 2.,
        };

        (
            ((render_width as f32 / 2. / TILE_SIZE as f32 / zoom).ceil() as usize) + extra,
            ((render_height as f32 / 2. / TILE_SIZE as f32 / zoom).ceil() as usize) + extra,
        )
    }

    fn update_entities<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();

        let (x, y) = (self.position.0 as usize, self.position.1 as usize);
        let (w_range, h_range) = self.get_render_tiles(1, engine.get_world_camera_zoom());
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

        self.render_size = (width as usize, height as usize);
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
        for i in [0, 3].iter() {
            match self.map_layer_textures[*i] {
                RenderTarget::Uninitialized => {
                    todo!()
                }
                RenderTarget::Dirty { texture_id } => {
                    for (y, x) in iproduct!(0..100, 0..100) {
                        let tile = &self.current_map.tiles[x][y];
                        let world_x = (x * TILE_SIZE) as u16;
                        let world_y = ((y * TILE_SIZE) - TILE_SIZE) as u16;
                        let position = Position::new(world_x, world_y, calculate_z(*i, x, y));

                        if tile.graphics[*i] != 0 {
                            self.draw_grh(
                                engine,
                                tile.graphics[*i] as u32,
                                position,
                                Target::Texture { id: texture_id },
                            );
                        }
                    }
                    self.map_layer_textures[*i] = RenderTarget::Ready { texture_id };
                }
                RenderTarget::Ready { .. } => {}
            }
        }
        self.draw_map_layer(engine, 0, [255, 255, 255, 255]);

        let (x, y) = (
            self.position.0.round() as usize,
            self.position.1.round() as usize,
        );
        let (w_range, h_range) = self.get_render_tiles(5, engine.get_world_camera_zoom());
        let (y_start, y_end) = (y.saturating_sub(h_range), min(y + h_range, MAP_HEIGHT));
        let (x_start, x_end) = (x.saturating_sub(w_range), min(x + w_range, MAP_WIDTH));
        for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
            let tile = &self.current_map.tiles[x][y];
            let world_x = (x * TILE_SIZE) as u16;
            let world_y = ((y * TILE_SIZE) - TILE_SIZE) as u16;

            for layer in [1, 2].iter() {
                if tile.graphics[*layer] != 0 {
                    let z = Z_ORDERING[*layer][x][y];
                    if self.draw_map {
                        let position = Position::new(world_x, world_y, z);
                        self.draw_grh(
                            engine,
                            tile.graphics[*layer] as u32,
                            position,
                            Target::World,
                        );
                    };
                }
            }
        }

        let mut visible_entities = 0;
        let (w_range, h_range) = self.get_render_tiles(1, engine.get_world_camera_zoom());
        let (y_start, y_end) = (y.saturating_sub(h_range), min(y + h_range, MAP_HEIGHT));
        let (x_start, x_end) = (x.saturating_sub(w_range), min(x + w_range, MAP_WIDTH));

        for (y, x) in iproduct!(y_start..y_end, x_start..x_end) {
            let tile = &self.current_map.tiles[x][y];

            if let Some(user) = tile.user {
                visible_entities += 1;
                let entity = &mut self.entities[user];
                if self.draw_entities {
                    entity.draw(engine, &self.resources);
                }
                if self.draw_names {
                    entity.draw_name(engine);
                }
            }
        }

        let text = engine.parse_text(
            Self::TAHOMA_ID,
            &format!("visible entities: {visible_entities}"),
            Orientation::Center,
        );
        engine.draw_text(
            Self::TAHOMA_ID,
            DrawText {
                text: &text.unwrap(),
                position: Position {
                    x: 50,
                    y: 30,
                    z: 1.,
                },
                color: [255, 255, 255, 255],
            },
            Target::UI,
        );

        let trigger = self.current_map.tiles[x][y].trigger;
        let text = engine.parse_text(
            Self::TAHOMA_ID,
            &format!("trigger: {trigger}"),
            Orientation::Center,
        );
        engine.draw_text(
            Self::TAHOMA_ID,
            DrawText {
                text: &text.unwrap(),
                position: Position {
                    x: 50,
                    y: 45,
                    z: 1.,
                },
                color: [255, 255, 255, 255],
            },
            Target::UI,
        );

        let trigger = self.current_map.tiles[x][y].trigger;
        let under_roof = trigger == 1 || trigger >= 20;
        let color = if under_roof {
            [255, 255, 255, 100]
        } else {
            [255, 255, 255, 255]
        };
        self.draw_map_layer(engine, 3, color);
    }

    fn draw_map_layer<E: GameEngine>(&mut self, engine: &mut E, i: usize, color: Color) {
        if let RenderTarget::Ready { texture_id } = self.map_layer_textures[i] {
            let viewport = engine.get_world_camera_viewport();
            let position = engine.get_world_camera_position();
            let x = position.x - viewport.width / 2.;
            let y = position.y - viewport.height / 2.;
            let inverted_y = ((100. - self.position.1) * TILE_SIZE as f32) - viewport.height / 2.;

            let source = [
                x as u16,
                inverted_y as u16,
                viewport.width as u16,
                viewport.height as u16,
            ];

            engine.draw_image(
                texture_id,
                DrawImage {
                    position: Position::new(x as u16, y as u16, calculate_z(i, 0, 0)),
                    color,
                    source,
                    index: texture_id as u32,
                },
                Target::World,
            );
        }
    }

    fn draw_grh<E: GameEngine>(
        &self,
        engine: &mut E,
        image_id: u32,
        position: engine::draw::Position,
        target: Target,
    ) {
        if let Some(image) = &self.resources.images[image_id as usize] {
            self.draw_image(engine, image, position, target);
        }
    }

    fn draw_image<E: GameEngine>(
        &self,
        engine: &mut E,
        image: &Image,
        mut position: engine::draw::Position,
        target: Target,
    ) {
        let image_num = image.file_num;
        position.x -= (image.width as f32 / 2.) as u16;

        engine.draw_image(
            image_num,
            DrawImage {
                position,
                color: [255, 255, 255, 255],
                source: [image.x, image.y, image.width, image.height],
                index: image_num as u32,
            },
            target,
        );
    }

    fn draw_ui<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();

        let text = engine.parse_text(
            Self::TAHOMA_ID,
            &format!("FPS: {:.2}", 1. / delta.as_secs_f32()),
            Orientation::Center,
        );

        engine.draw_text(
            Self::TAHOMA_ID,
            DrawText {
                text: &text.unwrap(),
                position: Position {
                    x: 50,
                    y: 15,
                    z: 1.,
                },
                color: [255, 255, 255, 255],
            },
            Target::UI,
        );
    }
}

fn calculate_z(layer: usize, x: usize, y: usize) -> f32 {
    match layer {
        0 => 0.,
        3 => 1.,
        i => (i as f32 * 1000. + (100. - y as f32) * 10. - x as f32) / 4000.,
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
