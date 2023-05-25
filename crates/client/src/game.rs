use std::sync::Arc;

use ao::Map;
use macroquad::prelude::*;

use crate::app::resources::Resources;

mod draw;
mod update;

pub struct Game {
    resources: Arc<Resources>,
    map: Map,
    map_render_targets: [(RenderTarget, bool); 4],
    map_static_camera: Camera2D,
    world_camera: Camera2D,
    ui_camera: Camera2D,
    position: Vec2,
}

impl Game {
    pub fn new(resources: Arc<Resources>) -> Self {
        let map = ao::ao_20::maps::parse::parse_map("./assets/maps", 1).expect("can parse map");
        let map_static_camera = create_map_static_camera();
        let world_camera = create_world_camera();
        let ui_camera = create_ui_camera();
        let position = vec2(50., 50.);

        Self {
            resources,
            map,
            map_render_targets: [
                (render_target(3200, 3200), false),
                (render_target(3200, 3200), false),
                (render_target(3200, 3200), false),
                (render_target(3200, 3200), false),
            ],
            position,
            map_static_camera,
            world_camera,
            ui_camera,
        }
    }

    pub async fn update(&mut self) {
        self.update_camera_viewport();
        self.update_position();
        self.world_camera.target = vec2(self.position.x * 32., self.position.y * 32.);
    }

    pub async fn render(&mut self) {
        self.render_ui();
        self.render_world().await;
    }

    async fn render_world(&mut self) {
        let coords = (
            self.position.x.floor() as usize,
            self.position.y.floor() as usize,
        );
        set_camera(&self.world_camera);
        self.draw_map(coords).await;
    }

    fn render_ui(&self) {
        set_camera(&self.ui_camera);
        self.draw_interface();
        self.draw_fps();
        self.draw_position();
    }

    fn update_camera_viewport(&mut self) {
        let world_size_px = 15. * 32.;
        let aspect_ratio_x = screen_width() / 800.;
        let aspect_ratio_y = screen_height() / 600.;

        self.world_camera.viewport = Some((
            (10. * aspect_ratio_x).round() as i32,
            (10. * aspect_ratio_y).round() as i32,
            (world_size_px * aspect_ratio_x) as i32,
            (world_size_px * aspect_ratio_y) as i32,
        ));
    }
}

fn create_ui_camera() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800., 600.0))
}

fn create_world_camera() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, 480., 480.))
}

fn create_map_static_camera() -> Camera2D {
    let (map_width, map_height) = (100 * 32, 100 * 32);
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, map_width as f32, map_height as f32));
    camera.render_target = Some(render_target(map_width, map_height));
    camera.zoom.y = -camera.zoom.y;
    camera
}
