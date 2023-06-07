use std::time::{Duration, Instant};

use font::RESERVED_ID;
use pollster::block_on;
use roma::{get_roma, get_state, init_roma};
pub use wgpu::PresentMode;
pub use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
pub use winit_input_helper::WinitInputHelper;

mod camera;
mod font;
mod renderer;
pub mod roma;
mod state;

#[derive(Default, Debug, Copy, Clone)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Default, Debug)]
pub struct DrawImageParams {
    pub texture_id: usize,
    pub x: usize,
    pub y: usize,
    pub z: f32,
    pub color: [f32; 4],
    pub source: Option<Rect>,
    pub flip_y: bool,
}

pub fn draw_image(params: DrawImageParams) {
    let roma = get_roma();
    roma.image_renderer.queue(params);
}

#[derive(Debug, Default, Clone)]
pub struct DrawTextParams<'s> {
    pub text: &'s str,
    pub x: usize,
    pub y: usize,
    pub z: f32,
    pub color: [f32; 4],
}

pub fn draw_text(params: DrawTextParams) {
    let roma = get_roma();

    let (data, total_width) = roma.fonts.parse(params.text);
    let offset_x = total_width / 2;
    for (x, y, source) in data {
        let x = params
            .x
            .saturating_add_signed(*x as isize)
            .saturating_sub(offset_x);
        let y = params.y.saturating_add_signed(*y as isize);
        roma.image_renderer.queue(DrawImageParams {
            texture_id: RESERVED_ID,
            x,
            y,
            z: params.z,
            color: params.color,
            source: Some(*source),
            flip_y: false,
        });
    }
}

pub fn set_camera_position(x: usize, y: usize) {
    let roma = get_roma();
    roma.camera2d.set_position(x, y);
}

pub fn get_input() -> &'static WinitInputHelper {
    let roma = get_roma();
    roma.input()
}

pub fn get_delta() -> Duration {
    let roma = get_roma();
    roma.get_delta()
}

pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub title: String,
    pub textures_folder: String,
    pub present_mode: wgpu::PresentMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Roma".to_string(),
            present_mode: wgpu::PresentMode::AutoNoVsync,
            textures_folder: "assets/textures".to_string(),
        }
    }
}

pub fn run_game(settings: Settings, mut game_loop: impl FnMut() + 'static) {
    block_on(async {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(settings.title.clone())
            .with_inner_size(winit::dpi::PhysicalSize::new(
                settings.width as u32,
                settings.height as u32,
            ))
            .build(&event_loop)
            .expect("> Roma > couldn't create window");
        let this_window_id = window.id();

        init_roma(window, settings).await;

        let mut last_tick = Instant::now();
        event_loop.run(move |window_event, _, control_flow| match window_event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == this_window_id => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    get_roma().resize(physical_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    get_roma().resize(new_inner_size);
                }
                _ => {
                    get_roma().input().update(&window_event);
                }
            },
            Event::RedrawRequested(window_id) if window_id == this_window_id => {
                let now = Instant::now();
                let delta = now - last_tick;
                get_roma().set_delta(delta);
                last_tick = now;

                game_loop();

                match get_roma().render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let roma = get_roma();
                        let new_size = get_state().size;
                        roma.resize(&new_size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => println!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                get_state().window.request_redraw();
            }
            _ => {}
        });
    });
}
