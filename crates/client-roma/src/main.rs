use client_roma::App;
use roma_wgpu::block_on;
use roma_wgpu::run;

mod settings;

fn main() {
    let app = App::default();
    block_on(run(move |roma| {
        app.update_camera(roma);
        app.draw_map(roma);
    }));
}
