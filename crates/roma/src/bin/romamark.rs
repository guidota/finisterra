use rand::Rng;
use roma::{draw_image_multiple, set_camera_position, VirtualKeyCode};
use roma::{draw_text, get_camera_size, get_delta, DrawImageParams, DrawTextParams};
use smol_str::SmolStr;

#[derive(Debug)]
struct Instance {
    position: [f32; 3],
    velocity: [f32; 2],
    color: [u8; 4],
}

impl Instance {
    fn random(rng: &mut impl Rng, velocity: [f32; 2], color: [u8; 4], z: f32) -> Self {
        Self {
            position: [
                rng.gen_range(0..WIDTH) as f32,
                rng.gen_range(0..HEIGHT) as f32,
                z,
            ],
            velocity,
            color,
        }
    }
}

struct State {
    instances: Vec<Instance>,
    staging: Vec<DrawImageParams>,
}

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let settings = roma::Settings {
        // textures_folder: "assets".to_string(),
        width: WIDTH,
        height: HEIGHT,
        present_mode: wgpu::PresentMode::AutoVsync,
        ..Default::default()
    };
    roma::run_game(
        settings,
        || {
            let mut rng = rand::thread_rng();
            let mut state = State {
                instances: vec![],
                staging: vec![],
            };

            for _ in 0..10 {
                spawn_wave(&mut state, &mut rng, 10000);
            }

            state
        },
        |state| {
            let input = roma::get_input();

            if input.key_held(VirtualKeyCode::Space) {
                spawn_wave(state, &mut rand::thread_rng(), 1000);
            }

            let delta = get_delta().as_secs_f32();
            let (viewport_x, viewport_y) = get_camera_size();
            // viewport_x /= 2.;
            // viewport_y /= 2.;
            set_camera_position(viewport_x / 2., viewport_y / 2.);

            const GRAVITY: f32 = -9.8 * 100.0;

            if state.staging.len() < state.instances.len() {
                state
                    .staging
                    .resize(state.instances.len() * 2, DrawImageParams::default());
            }
            for (i, instance) in state.instances.iter_mut().enumerate() {
                instance.position[0] += instance.velocity[0] * delta;
                instance.position[1] += instance.velocity[1] * delta;

                instance.velocity[1] += GRAVITY * delta;

                const HX: f32 = 16.;
                const HY: f32 = 16.;

                let x = instance.position[0];
                let y = instance.position[1];
                let vx = instance.velocity[0];
                let vy = instance.velocity[1];

                if (vx > 0.0 && x + HX > viewport_x) || (vx <= 0.0 && x - HX < 0.) {
                    instance.velocity[0] = -vx;
                }

                if vy < 0.0 && y - HY < 0. {
                    instance.velocity[1] = -vy;
                }

                if instance.position[1] + HY > viewport_y && instance.velocity[1] > 0.0 {
                    instance.velocity[1] = 0.0;
                }

                state.staging[i].x = instance.position[0] as u16;
                state.staging[i].y = instance.position[1] as u16;
                state.staging[i].z = instance.position[2];
                state.staging[i].color = instance.color;
            }
            draw_image_multiple(3, &mut state.staging[..state.instances.len()]);

            let fps = 1. / delta;
            draw_text(DrawTextParams {
                text: SmolStr::new(format!("fps: {:.2}", fps)),
                position: [50., 10., 1.],
                color: [0, 255, 0, 255],
            });
            draw_text(DrawTextParams {
                text: SmolStr::new(format!("instances: {}", state.instances.len())),
                position: [50., 0., 1.],
                color: [0, 255, 0, 255],
            });
        },
        |_, _| {},
    );
}

fn spawn_wave(state: &mut State, rng: &mut impl Rng, wave_size: usize) {
    const MAX_VELOCITY: f32 = 750.;
    let velocity_x = rng.gen::<f32>() * MAX_VELOCITY - (MAX_VELOCITY * 0.5);
    let color = [
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        255,
    ];
    for i in 0..wave_size {
        let z = (state.instances.len() + i) as f32 * 0.0000001;
        state
            .instances
            .push(Instance::random(rng, [velocity_x, 0.], color, z));
    }
}
