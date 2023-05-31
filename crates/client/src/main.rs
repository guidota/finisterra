use client::{app::window::*, app::App, settings::Settings};
use macroquad::prelude::*;
use macroquad_profiler::ProfilerParams;

#[macroquad::main(window_conf())]
async fn main() {
    let settings = Settings::default();
    let mut app = App::new(settings).await;

    loop {
        clear_background(BLACK);

        app.update().await;
        app.render().await;
        macroquad_profiler::profiler(ProfilerParams::default());

        next_frame().await;
    }
}
