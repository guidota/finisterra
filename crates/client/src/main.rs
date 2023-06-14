use client::{Finisterra, RENDER_H, RENDER_W};
use roma::{run_game, Settings};

mod settings;

fn main() {
    let mut game = Finisterra::ao_20();
    let base_path = "./assets/ao_20/graphics/".to_string();
    let settings = Settings {
        width: RENDER_W,
        height: RENDER_H,
        title: "Finisterra".to_string(),
        present_mode: roma::PresentMode::AutoNoVsync,
        textures_folder: base_path,
    };
    run_game(settings, move || game.game_loop());
}
