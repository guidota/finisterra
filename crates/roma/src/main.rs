use roma::{draw_image, draw_text, get_delta, set_camera_position};

fn main() {
    let settings = roma::Settings {
        textures_folder: "art/".to_string(),
        ..Default::default()
    };
    roma::run_game(settings, || {
        set_camera_position(400, 300);
        draw_image(roma::DrawImageParams {
            texture_id: 0,
            color: [1., 1., 1., 1.],
            z: 0.2,
            ..Default::default()
        });
        draw_text(roma::DrawTextParams {
            text: "Pandora",
            color: [1., 0., 0., 1.],
            z: 0.5,
            x: 100,
            y: 100,
        });
        let delta = get_delta();
        draw_text(roma::DrawTextParams {
            text: "FPS: ",
            color: [1., 0., 0., 1.],
            z: 0.5,
            x: 20,
            y: 5,
        });
        draw_text(roma::DrawTextParams {
            text: &format!("{:.2}", 1. / delta.as_secs_f32()),
            color: [1., 0., 0., 1.],
            z: 0.5,
            x: 60,
            y: 5,
        });
    });
}
