use std::time::{Duration, Instant};

use graphics::{vec2::Vec2, Graphics};
use resources::camera::Camera2D;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub mod graphics;
pub mod render;
pub mod resources;

pub use pollster::*;
pub use wgpu::*;
pub use winit::*;
use winit_input_helper::WinitInputHelper;

pub struct Roma {
    pub graphics: Graphics,
    pub input: WinitInputHelper,
    pub camera: Camera2D,
}

impl Roma {
    pub async fn new(window: Window) -> Self {
        let input = WinitInputHelper::new();
        let camera = Camera2D::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );
        Self {
            graphics: Graphics::new(window).await,
            camera,
            input,
        }
    }

    pub fn set_camera(&mut self, camera: Camera2D) {
        self.camera = camera;
    }

    pub fn get_camera_position(&mut self) -> Vec2 {
        self.camera.position
    }

    pub fn set_camera_position(&mut self, position: Vec2) {
        self.camera.set_position(position);
        self.graphics
            .set_camera_projection(self.camera.build_view_projection_matrix());
    }
}

pub trait Game {
    fn update(&mut self, roma: &mut Roma, delta: Duration);
}

pub async fn run<G>(mut game: G)
where
    G: Game + 'static,
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(dpi::PhysicalSize::new(800., 600.))
        .build(&event_loop)
        .unwrap();

    let mut app = Roma::new(window).await;
    let mut last_tick = Instant::now();
    event_loop.run(move |window_event, _, control_flow| {
        match window_event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.graphics.window().id() => match event {
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
                    app.graphics.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    app.graphics.resize(new_inner_size);
                }
                _ => {
                    app.input.update(&window_event);
                }
            },
            Event::RedrawRequested(window_id) if window_id == app.graphics.window().id() => {
                let now = Instant::now();
                let delta = now - last_tick;
                last_tick = now;

                game.update(&mut app, delta);
                match app.graphics.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let new_size = *app.graphics.size();
                        app.graphics.resize(&new_size);
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => println!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                app.graphics.window().request_redraw();
            }
            _ => {}
        }
    });
}
