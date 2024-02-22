use engine::{
    camera::{Viewport, Zoom},
    engine::GameEngine,
};

use crate::ui::UI;

use super::GameScreen;

pub struct WorldScreen {
    ui: WorldUI,
}

pub struct WorldUI {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

const SCREEN_WIDTH: u16 = 800;
const SCREEN_HEIGHT: u16 = 540;

const TILE_SIZE: u16 = 32;
const WORLD_RENDER_WIDTH: u16 = 17 * TILE_SIZE; // 17 tiles
const WORLD_RENDER_HEIGHT: u16 = 16 * TILE_SIZE; // 16 tiles

impl GameScreen for WorldScreen {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        self.prepare_viewports(context.engine);
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        todo!()
    }
}

impl WorldScreen {
    fn prepare_viewports<E: GameEngine>(&mut self, engine: &mut E) {
        let size = engine.get_window_size();
        let zoom = if size.height >= (SCREEN_HEIGHT * 2) && size.width >= (SCREEN_WIDTH * 2) {
            engine.set_camera_zoom(Zoom::Double);
            2
        } else {
            engine.set_camera_zoom(Zoom::None);
            1
        };

        let x_start = std::cmp::max(0, (size.width / zoom - SCREEN_WIDTH) / 2);
        let y_start = std::cmp::max(0, (size.height / zoom - SCREEN_HEIGHT) / 2);
        self.ui.x = x_start;
        self.ui.y = y_start;
        self.ui.width = SCREEN_WIDTH;
        self.ui.height = SCREEN_HEIGHT;

        // TODO: should we use entire screen for UI but only draw calls inside this rect?
        engine.set_ui_camera_viewport(Viewport {
            x: x_start as f32,
            y: y_start as f32,
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
        });

        engine.set_world_camera_viewport(Viewport {
            x: x_start as f32 + 14.,
            y: y_start as f32 + 14.,
            width: WORLD_RENDER_WIDTH as f32,
            height: WORLD_RENDER_HEIGHT as f32,
        });
    }
}

impl UI for WorldUI {
    fn update<E: engine::engine::GameEngine>(&mut self, engine: &mut E) {
        todo!()
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, engine: &mut E) {
        todo!()
    }
}
