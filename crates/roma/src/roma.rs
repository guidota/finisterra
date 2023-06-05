use std::time::{Duration, Instant};

use glyph_brush::{ab_glyph::PxScale, Layout, Section, Text};
use pollster::block_on;
use rustc_hash::FxHashMap;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    draw::{DrawImageParams, DrawImageStrictParams, DrawTextParams},
    render::{
        textures::TextureID,
        vertex::{draw_params_to_vertex, Vertex},
        Batch, Instructions, Renderer,
    },
    resources::{camera::Camera2D, text_brush::TextBrush},
    settings::Settings,
};

pub struct Roma {
    pub(crate) renderer: Renderer,
    camera: Camera2D,
    input: WinitInputHelper,

    /// The current frame's draw instructions per texture
    image_instructions: FxHashMap<usize, Vec<DrawImageStrictParams>>,
    text_brush: TextBrush,
}

impl Roma {
    pub fn input(&self) -> &winit_input_helper::WinitInputHelper {
        &self.input
    }

    pub fn set_camera_position(&mut self, x: usize, y: usize) {
        self.camera.set_position(x, y);
    }

    pub fn draw_image(&mut self, draw_params: DrawImageParams) {
        self.renderer.prepare_texture(draw_params.texture_id);
        match self
            .renderer
            .get_texture(&TextureID::Image(draw_params.texture_id))
        {
            Some(texture) => {
                let texture_id = draw_params.texture_id;
                let draw_params = draw_params.to_strict(texture);
                self.image_instructions
                    .entry(texture_id)
                    .or_default()
                    .push(draw_params);
            }
            None => {
                println!("> draw_texture > texture not found or unable to load");
            }
        }
    }

    pub fn draw_text(&mut self, draw_params: DrawTextParams) {
        let align = match draw_params.align {
            crate::draw::TextAlign::Left => glyph_brush::HorizontalAlign::Left,

            crate::draw::TextAlign::Center => glyph_brush::HorizontalAlign::Center,

            crate::draw::TextAlign::Right => glyph_brush::HorizontalAlign::Right,
        };
        let layout = Layout::default_single_line().h_align(align);
        let color = draw_params.color;
        let section = Section::new()
            .with_screen_position((draw_params.x as f32, draw_params.y as f32))
            .with_layout(layout)
            .add_text(
                Text::new(draw_params.text)
                    .with_scale(PxScale::from(draw_params.size as f32))
                    .with_z(draw_params.z)
                    .with_color([
                        color.r as f32,
                        color.g as f32,
                        color.b as f32,
                        color.a as f32,
                    ]),
            );
        self.text_brush.queue(section);
    }

    fn prepare_instructions(&mut self) -> Instructions {
        self.renderer.update_camera(&self.camera);
        self.renderer.update_glyphs(&mut self.text_brush);

        let mut vertices = vec![];
        let mut batches = vec![];

        for (texture_id, draw_params) in self.image_instructions.iter() {
            batches.push(Batch {
                texture_id: TextureID::Image(*texture_id),
                size: draw_params.len() as u32,
            });
            vertices.append(&mut self.prepare_vertices(draw_params));
        }
        self.image_instructions.clear();

        let text_instructions = self.text_brush.get_instructions();
        for (font_id, ref text_vertices) in text_instructions {
            batches.push(Batch {
                texture_id: TextureID::Glyph(font_id.0),
                size: vertices.len() as u32,
            });
            vertices.extend_from_slice(text_vertices);
        }

        Instructions { vertices, batches }
    }

    fn prepare_vertices(&self, batches: &[DrawImageStrictParams]) -> Vec<Vertex> {
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

            let mut renderer = Renderer::new(&settings.renderer, window).await;
            let camera = Camera2D::new(settings.window.window_width, settings.window.window_height);
            let input = WinitInputHelper::new();
            let text_brush = TextBrush::tahoma();

            renderer.recreate_texture(TextureID::Glyph(0), text_brush.dimensions());

            let mut roma = Roma {
                renderer,
                camera,
                input,
                image_instructions: FxHashMap::default(),
                text_brush,
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
