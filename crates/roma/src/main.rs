use roma::draw_image;

fn main() {
    let settings = roma::Settings::default();
    roma::run_game(settings, || {
        draw_image(roma::DrawImageParams {
            texture_id: 0,
            ..Default::default()
        });
    });
}
