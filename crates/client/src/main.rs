use client::{Finisterra, RENDER_H, RENDER_W};
use roma::{run_game, Settings};

mod settings;

fn main() {
    let mut game = Finisterra::default();
    let base_path = "./assets/ao_20/graphics/".to_string();
    // let window_settings = WindowSettingsBuilder::default()
    //     .window_title("Roma")
    //     .window_width(RENDER_W)
    //     .window_height(RENDER_H)
    //     .build()
    //     .unwrap();
    // let renderer_settings = RendererSettingsBuilder::default()
    //     .present_mode(roma::PresentMode::AutoNoVsync)
    //     .base_path(base_path)
    //     .build()
    //     .unwrap();
    // let settings = SettingsBuilder::default()
    //     .window(window_settings)
    //     .renderer(renderer_settings)
    //     .build()
    //     .unwrap();
    let settings = Settings {
        width: RENDER_W,
        height: RENDER_H,
        title: "Finisterra".to_string(),
        present_mode: roma::PresentMode::AutoNoVsync,
        textures_folder: base_path,
    };
    run_game(settings, move || game.game_loop());
}
