use std::time::Duration;

use roma::{
    draw::DrawParams,
    roma::{Game, Roma},
    settings::{RendererSettingsBuilder, SettingsBuilder, WindowSettingsBuilder},
};

struct MyGame {}

impl Game for MyGame {
    fn update(&mut self, roma: &mut Roma, _delta: Duration) {
        roma.set_camera_position(0, 0);
        roma.draw_texture(DrawParams {
            texture_id: 1,
            flip_y: true,
            ..Default::default()
        });

        roma.draw_texture(DrawParams {
            texture_id: 1,
            x: 200,
            y: 200,
            flip_y: true,
            ..Default::default()
        });
        roma.draw_texture(DrawParams {
            texture_id: 1,
            x: 400,
            y: 400,
            flip_y: true,
            ..Default::default()
        });
        //
        // roma.graphics.draw_texture(
        //     4.to_string(),
        //     1.to_string(),
        //     0.,
        //     100.,
        //     Color::WHITE,
        //     Some(DrawTextureParams {
        //         flip_y: true,
        //         // source: Some(Rect::new(0., 0., 32., 32.)),
        //         ..Default::default()
        //     }),
        // );
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
