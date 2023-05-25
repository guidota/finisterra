use macroquad::{miniquad::conf::*, prelude::*};

pub const RENDER_W: f32 = 800.0;
pub const RENDER_H: f32 = 600.0;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Finisterra".to_owned(),
        // fullscreen: true,
        window_width: RENDER_W as i32,
        window_height: RENDER_H as i32,
        // platform: Platform {
        // linux_backend: LinuxBackend::WaylandOnly,
        // ..Default::default()
        // },
        ..Default::default()
    }
}
