use client::Finisterra;
use roma::{
    roma::Roma,
    settings::{RendererSettingsBuilder, SettingsBuilder, WindowSettingsBuilder},
};

mod settings;

fn main() {
    let game = Finisterra::default();
    let base_path = "./assets/99z/graphics/".to_string();
    let window_settings = WindowSettingsBuilder::default()
        .window_title("Roma")
        .window_width(800_usize)
        .window_height(600_usize)
        .build()
        .unwrap();
    let renderer_settings = RendererSettingsBuilder::default()
        .present_mode(roma::PresentMode::AutoNoVsync)
        .base_path(base_path)
        .build()
        .unwrap();
    let settings = SettingsBuilder::default()
        .window(window_settings)
        .renderer(renderer_settings)
        .build()
        .unwrap();
    Roma::run_game(settings, game);
}
