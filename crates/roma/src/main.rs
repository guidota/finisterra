use roma::{draw_image, draw_text, set_camera_position};

fn main() {
    let settings = roma::Settings {
        textures_folder: "art/".to_string(),
        ..Default::default()
    };
    roma::run_game(settings, || {
        set_camera_position(400, 300);
        // draw_image(roma::DrawImageParams {
        //     texture_id: 0,
        //     color: [1., 1., 1., 1.],
        //     z: 0.6,
        //     ..Default::default()
        // });
        draw_text(roma::DrawTextParams {
            text: "Pandora",
            color: [1., 0., 0., 1.],
            z: 0.5,
            ..Default::default()
        });
    });
}
