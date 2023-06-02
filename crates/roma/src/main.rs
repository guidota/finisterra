use std::time::Duration;

use pollster::block_on;
use roma::{graphics::textures::DrawParams, run, Game, Roma};

struct MyGame {}

impl Game for MyGame {
    fn update(&mut self, roma: &mut Roma, _delta: Duration) {
        roma.graphics.load_texture(&1);
        roma.graphics.load_texture(&2);
        roma.graphics.draw_texture(
            0,
            DrawParams {
                texture_id: 1,
                ..Default::default()
            },
        );

        roma.graphics.draw_texture(
            1,
            DrawParams {
                texture_id: 2,
                x: 200,
                y: 200,
                ..Default::default()
            },
        );
        roma.graphics.draw_texture(
            2,
            DrawParams {
                texture_id: 2,
                x: 400,
                y: 400,
                ..Default::default()
            },
        );
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
    block_on(run(base_path, game));
}
