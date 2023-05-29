use std::{collections::HashMap, sync::Arc};

use ao::Map;
use macroquad::prelude::*;

use crate::{app::resources::Resources, settings::Settings};

use self::{
    camera::{create_map_static_camera, create_ui_camera, create_world_camera},
    entity::Entity,
};

mod camera;
mod draw;
mod entity;
mod update;

pub struct Game {
    resources: Arc<Resources>,
    map: Map,
    map_render_targets: [(RenderTarget, bool); 4],
    map_static_camera: Camera2D,
    world_camera: Camera2D,
    ui_camera: Camera2D,
    position: Vec2,
    entities: HashMap<usize, Entity>,
    screen_size: Vec2,
    screen_size_dirty: bool,
    settings: Settings,
    test: bool,
}

impl Game {
    pub fn new(settings: Settings, resources: Arc<Resources>) -> Self {
        let mut map = ao::ao_20::maps::parse::parse_map("./assets/maps", 1).expect("can parse map");
        let map_static_camera = create_map_static_camera();
        let world_camera = create_world_camera();
        let ui_camera = create_ui_camera();
        let position = vec2(50., 50.);

        let mut entities = HashMap::new();
        for id in 1..=1 {
            let random = Entity::random(&resources);
            let Vec2 { x, y } = random.position;
            map.tiles[x as usize][y as usize].char_index = id;
            entities.insert(id, random);
        }

        Self {
            resources,
            map,
            map_render_targets: [
                (render_target(3200, 3200), false),
                (render_target(1, 1), false),
                (render_target(1, 1), false),
                (render_target(3200, 3200), false),
            ],
            position,
            map_static_camera,
            world_camera,
            ui_camera,
            entities,
            screen_size: vec2(screen_width(), screen_height()),
            screen_size_dirty: false,
            settings,
            test: false,
        }
    }

    pub async fn update(&mut self) {
        self.update_screen_size();
        self.update_position();
        self.world_camera.target = vec2(self.position.x * 32., self.position.y * 32.);
    }

    pub async fn render(&mut self) {
        if self.settings.draw_ui {
            self.render_ui();
        }

        self.render_world().await;
        if !self.test && self.settings.preload_textures {
            // build_textures_atlas();
            self.test = true;
        }
    }

    async fn render_world(&mut self) {
        set_camera(&self.world_camera);
        self.draw_world().await;
    }

    fn render_ui(&self) {
        set_camera(&self.ui_camera);
        self.draw_interface();
        self.draw_fps();
        self.draw_position();
    }

    fn update_screen_size(&mut self) {
        if screen_width() != self.screen_size.x || screen_height() != self.screen_size.y {
            self.screen_size_dirty = true;
            self.screen_size = vec2(screen_width(), screen_height());
            // let world_size_px = 15. * 32.;
            // let aspect_ratio_x = screen_width() / 800.;
            // let aspect_ratio_y = screen_height() / 600.;

            // self.world_camera.viewport = Some((
            //     (10. * aspect_ratio_x).round() as i32,
            //     (10. * aspect_ratio_y).round() as i32,
            //     (world_size_px * aspect_ratio_x) as i32,
            //     (world_size_px * aspect_ratio_y) as i32,
            // ));
        } else {
            self.screen_size_dirty = false;
        }
    }
}
