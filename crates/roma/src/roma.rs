use std::{
    iter::once,
    time::{Duration, Instant},
};

use pollster::block_on;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    camera::Camera2D,
    renderer::image::ImageRenderer,
    renderer::text::{Fonts, RESERVED_ID},
    state::State,
    DrawImageParams, DrawTextParams,
};

pub struct Roma {
    input: WinitInputHelper,
    image_renderer: ImageRenderer,

    depth_texture_view: wgpu::TextureView,
    camera2d: Camera2D,
    fonts: Fonts,

    staging: Vec<DrawImageParams>,
    delta: Duration,
}

impl Roma {
    pub async fn new(settings: Settings) -> Roma {
        let camera2d = Camera2D::new(settings.width, settings.height);
        let mut image_renderer = ImageRenderer::init(&settings.textures_folder);
        let input = WinitInputHelper::new();

        let depth_texture_view = Self::create_depth_texture();
        let (texture_id, font_texture) = Fonts::create_font_texture();
        image_renderer.add_texture(texture_id, &font_texture);

        let fonts = Fonts::init();

        Self {
            input,
            image_renderer,
            camera2d,
            depth_texture_view,
            delta: Duration::from_secs(0),
            fonts,
            staging: Vec::with_capacity(ImageRenderer::INITIAL_SPRITES),
        }
    }

    fn create_depth_texture() -> wgpu::TextureView {
        let state = get_state();

        let size = wgpu::Extent3d {
            width: state.config.width,
            height: state.config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Depth24PlusStencil8],
        };
        let texture = state.device.create_texture(&desc);

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub(crate) fn input(&mut self) -> &mut WinitInputHelper {
        &mut self.input
    }

    pub(crate) fn get_delta(&self) -> Duration {
        self.delta
    }

    pub(crate) fn set_delta(&mut self, delta: Duration) {
        self.delta = delta;
    }

    pub(crate) fn resize(&mut self, physical_size: &winit::dpi::PhysicalSize<u32>) {
        let state = get_state_mut();
        state.resize(*physical_size);
        self.depth_texture_view = Self::create_depth_texture();
    }

    fn update_camera(&mut self) {
        let projection = self.camera2d.build_view_projection_matrix();
        self.image_renderer.update_projection(projection);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.update_camera();
        let state = get_state();
        let frame = state.surface.get_current_texture()?;
        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: true,
                    }),

                    stencil_ops: None,
                }),
            });

            self.image_renderer.render_pass(&mut render_pass);
        }

        state.queue.submit(once(encoder.finish()));
        frame.present();
        Ok(())
    }
}

static mut ROMA: Option<Roma> = None;
static mut STATE: Option<State> = None;

pub(crate) async fn init_roma(window: Window, settings: Settings) {
    unsafe {
        STATE = Some(State::init(window, settings.present_mode).await);
        ROMA = Some(Roma::new(settings).await);
    }
}

pub(crate) fn get_roma() -> &'static mut Roma {
    unsafe { ROMA.as_mut().unwrap_or_else(|| panic!()) }
}

pub(crate) fn get_state() -> &'static State {
    unsafe { STATE.as_ref().unwrap_or_else(|| panic!()) }
}

pub(crate) fn get_state_mut() -> &'static mut State {
    unsafe { STATE.as_mut().unwrap_or_else(|| panic!()) }
}

pub fn draw_image(params: DrawImageParams) {
    let roma = get_roma();
    roma.image_renderer.queue(params);
}

pub fn draw_text(params: DrawTextParams) {
    let roma = get_roma();
    let Some((char_positions, total_width)) = roma.fonts.parse(params.text) else {return};

    let offset_x = *total_width as f32 / 2.;
    let [draw_x, draw_y, draw_z] = params.position;

    let chars_staging = &mut roma.staging;
    for char in char_positions {
        let x = char.screen_rect.x;
        let y = char.screen_rect.y;
        let source = [
            char.page_rect.x as f32,
            char.page_rect.y as f32,
            char.screen_rect.width as f32,
            char.screen_rect.height as f32,
        ];

        let x = draw_x + x as f32 - offset_x;
        let y = draw_y + y as f32;
        chars_staging.push(DrawImageParams {
            texture_id: RESERVED_ID,
            position: [x, y, draw_z],
            color: params.color,
            source: Some(source),
            flip_y: false,
        });
    }
    roma.image_renderer
        .queue_multiple(RESERVED_ID, chars_staging);
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
