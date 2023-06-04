use std::time::{Duration, Instant};

use pollster::block_on;
use rustc_hash::FxHashMap;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    draw::{DrawParams, DrawStrictParams},
    render::{
        vertex::{draw_params_to_vertex, Vertex},
        Batch, Instructions, Renderer,
    },
    resources::camera::Camera2D,
    settings::Settings,
};

pub struct Roma {
    pub(crate) renderer: Renderer,
    camera: Camera2D,
    input: WinitInputHelper,

    /// The current frame's draw instructions per texture
    frame_instructions: FxHashMap<usize, Vec<DrawStrictParams>>,
}

impl Roma {
    pub fn input(&self) -> &winit_input_helper::WinitInputHelper {
        &self.input
    }

    pub fn set_camera_position(&mut self, x: usize, y: usize) {
        self.camera.set_position(x, y);
    }

    pub fn draw_texture(&mut self, draw_params: DrawParams) {
        self.renderer.prepare_texture(draw_params.texture_id);
        match self.renderer.get_texture(&draw_params.texture_id) {
            Some(texture) => {
                let texture_id = draw_params.texture_id;
                let draw_params = draw_params.to_strict(texture);
                self.frame_instructions
                    .entry(texture_id)
                    .or_default()
                    .push(draw_params);
            }
            None => {
                println!("> draw_texture > texture not found or unable to load");
            }
        }
    }

    fn prepare_instructions(&mut self) -> Instructions {
        self.renderer.update_camera(&self.camera);
        let mut vertices = vec![];
        let mut batches = vec![];
        for (texture_id, draw_params) in self.frame_instructions.iter() {
            batches.push(Batch {
                texture_id: *texture_id,
                size: draw_params.len() as u32,
            });
            vertices.append(&mut self.prepare_vertices(draw_params));
        }
        Instructions { vertices, batches }
    }

    fn prepare_vertices(&self, batches: &[DrawStrictParams]) -> Vec<Vertex> {
        let mut vertices = Vec::with_capacity(batches.len() * 4);
        for param in batches {
            vertices.append(&mut draw_params_to_vertex(param));
        }
        vertices
    }

    pub fn run_game<G: Game + 'static>(settings: Settings, mut game: G) {
        block_on(async {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title(settings.window.window_title.clone())
                .with_inner_size(winit::dpi::PhysicalSize::new(
                    settings.window.window_width as u32,
                    settings.window.window_height as u32,
                ))
                .build(&event_loop)
                .expect("> Roma > couldn't create window");
            let renderer = Renderer::new(&settings.renderer, window).await;
            let camera = Camera2D::new(settings.window.window_width, settings.window.window_height);

            let input = WinitInputHelper::new();

            let mut roma = Roma {
                renderer,
                camera,
                input,
                frame_instructions: FxHashMap::default(),
            };

            let mut last_tick = Instant::now();
            event_loop.run(move |window_event, _, control_flow| {
                match window_event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == roma.renderer.window().id() => match event {
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
                            roma.renderer.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            roma.renderer.resize(new_inner_size);
                        }
                        _ => {
                            roma.input.update(&window_event);
                        }
                    },
                    Event::RedrawRequested(window_id)
                        if window_id == roma.renderer.window().id() =>
                    {
                        let now = Instant::now();
                        let delta = now - last_tick;
                        last_tick = now;

                        game.update(&mut roma, delta);

                        let instructions = roma.prepare_instructions();
                        match roma.renderer.render(instructions) {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                let new_size = *roma.renderer.size();
                                roma.renderer.resize(&new_size);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }

                            Err(wgpu::SurfaceError::Timeout) => println!("Surface timeout"),
                        }
                        roma.frame_instructions.clear();
                    }
                    Event::RedrawEventsCleared => {
                        // RedrawRequested will only trigger once, unless we manually request it.
                        roma.renderer.window().request_redraw();
                    }
                    _ => {}
                }
            });
        });
    }
}

pub trait Game {
    fn update(&mut self, roma: &mut Roma, delta: Duration);
}
