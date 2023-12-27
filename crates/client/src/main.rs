use client::{Finisterra, WINDOW_HEIGHT, WINDOW_WIDTH};
use engine::game::run_game;
use veril::Veril;

mod settings;

fn main() {
    let settings = engine::settings::Settings {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: "Finisterra".to_string(),
        vsync: true,
    };
    run_game::<Finisterra, Veril>(settings);
}
