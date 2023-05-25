use client::{app::window::*, app::App};
use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    let mut app = App::new().await;

    loop {
        clear_background(BLACK);

        app.update().await;
        app.render().await;

        next_frame().await;
    }
}
