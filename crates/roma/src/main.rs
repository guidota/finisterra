use std::time::Duration;

use roma::{
    draw::{DrawImageParams, DrawTextParams},
    roma::{Game, Roma},
    settings::{RendererSettingsBuilder, SettingsBuilder, WindowSettingsBuilder},
};

struct MyGame {}

impl Game for MyGame {
    fn update(&mut self, roma: &mut Roma, _delta: Duration) {
        roma.set_camera_position(0, 0);
        roma.draw_image(DrawImageParams {
            texture_id: 1,
            ..Default::default()
        });
        // roma.draw_image(DrawParams {
        //     texture_id: 1,
        //     x: 200,
        //     y: 200,
        //     ..Default::default()
        // });

        let draw_text_params = DrawTextParams {
            text: "Pandora",
            x: 0,
            y: 0,
            z: 1.0,
            size: 12,
            color: wgpu::Color::RED,
            flip_y: false,
            align: roma::draw::TextAlign::Center,
        };
        roma.draw_text(draw_text_params);
    }
}

fn main() {
    let game = MyGame {};
    let base_path = "./assets/99z/graphics/".to_string();
    let window_settings = WindowSettingsBuilder::default()
        .window_title("Roma")
        .window_width(800_usize)
        .window_height(600_usize)
        .build()
        .unwrap();
    let renderer_settings = RendererSettingsBuilder::default()
        .present_mode(wgpu::PresentMode::AutoNoVsync)
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
