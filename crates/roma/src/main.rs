use std::time::Duration;

use pollster::block_on;
use roma::{graphics::textures::DrawTextureParams, run, Game, Roma};

struct MyGame {}

impl Game for MyGame {
    fn update(&mut self, roma: &mut Roma, _delta: Duration) {
        roma.graphics
            .load_texture(1.to_string(), "./assets/99z/graphics/1.BMP");
        roma.graphics.load_texture(2.to_string(), "./assets/2.png");
        roma.graphics.draw_texture(
            0,
            1.to_string(),
            0.,
            0.,
            0,
            Some(DrawTextureParams {
                flip_y: true,
                // source: Some(Rect::new(0., 0., 32., 32.)),
                ..Default::default()
            }),
        );

        roma.graphics.draw_texture(
            1,
            2.to_string(),
            200.,
            200.,
            0,
            Some(DrawTextureParams {
                flip_y: true,
                // source: Some(Rect::new(0., 0., 32., 32.)),
                ..Default::default()
            }),
        );
        //
        roma.graphics.draw_texture(
            2,
            2.to_string(),
            400.,
            400.,
            0,
            Some(DrawTextureParams {
                flip_y: true,
                // source: Some(Rect::new(0., 0., 32., 32.)),
                ..Default::default()
            }),
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
    block_on(run(game));
}
