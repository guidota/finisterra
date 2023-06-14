use roma::{draw_image, draw_text, get_delta, set_camera_position};
use smol_str::SmolStr;

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
            position: [0., 0., 0.2],
            ..Default::default()
        });
        draw_text(roma::DrawTextParams {
            text: SmolStr::new("Pandora"),
            color: [1., 0., 0., 1.],
            position: [100., 100., 0.5],
        });
        let delta = get_delta();
        draw_text(roma::DrawTextParams {
            text: SmolStr::new("FPS: "),
            color: [1., 0., 0., 1.],
            position: [20., 5., 0.5],
        });
        draw_text(roma::DrawTextParams {
            text: SmolStr::new(format!("{:.2}", 1. / delta.as_secs_f32())),
            color: [1., 0., 0., 1.],
            position: [60., 5., 0.5],
        });
    });
}
