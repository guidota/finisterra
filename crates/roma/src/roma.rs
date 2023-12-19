use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use smol_str::SmolStr;
use yakui::ManagedTextureId;
use yakui_wgpu::SurfaceInfo;

use crate::{
    camera::{self, Camera2D},
    renderer::fonts::{Fonts, TEXT_ATLAS_TEXTURE_ID},
    renderer::sprite_batch::SpriteBatch,
    settings::Settings,
    state::State,
    Color, DrawImageParams, DrawTextParams,
};

pub struct Roma {
    input: winit_input_helper::WinitInputHelper,
    sprite_batch: SpriteBatch,

    depth_texture_view: wgpu::TextureView,
    multisample_texture_view: wgpu::TextureView,
    camera2d: Camera2D,
    fonts: Fonts,

    staging: Vec<DrawImageParams>,
    delta: Duration,

    // ui
    ui_yak: yakui::Yakui,
    ui_window: yakui_winit::YakuiWinit,
    ui_renderer: yakui_wgpu::YakuiWgpu,
}

impl Roma {
    pub async fn new(settings: Settings) -> Roma {
        let camera2d = Camera2D::new(settings.width as f32, settings.height as f32);
        let mut sprite_batch = SpriteBatch::init();
        let input = winit_input_helper::WinitInputHelper::new();

        let depth_texture_view = Self::create_depth_texture();
        let multisample_texture_view = Self::create_multisample_texture();
        let (texture_id, font_texture) = Fonts::create_font_texture();
        sprite_batch.add_texture(texture_id, &font_texture);

        let fonts = Fonts::init();

        let device = &get_state().device;
        let queue = &get_state().queue;
        let window = &get_state().window;

        let ui_renderer = yakui_wgpu::YakuiWgpu::new(device, queue);
        let ui_window = yakui_winit::YakuiWinit::new(window);
        let ui_yak = yakui::Yakui::new();

        Self {
            input,
            sprite_batch,
            camera2d,
            depth_texture_view,
            multisample_texture_view,
            delta: Duration::from_secs(0),
            fonts,
            staging: Vec::with_capacity(SpriteBatch::INITIAL_SPRITES),
            ui_renderer,
            ui_window,
            ui_yak,
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
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Depth24PlusStencil8],
        };
        let texture = state.device.create_texture(&desc);

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn create_multisample_texture() -> wgpu::TextureView {
        let state = get_state();
        let format = state.config.view_formats[0];

        let size = wgpu::Extent3d {
            width: state.config.width,
            height: state.config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("multisample_texture"),
            size,
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let texture = state.device.create_texture(&desc);

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn input(&mut self) -> &mut winit_input_helper::WinitInputHelper {
        &mut self.input
    }

    fn get_delta(&self) -> Duration {
        self.delta
    }

    fn set_delta(&mut self, delta: Duration) {
        self.delta = delta;
    }

    fn resize(&mut self, physical_size: &winit::dpi::PhysicalSize<u32>) {
        let state = get_state_mut();
        state.resize(*physical_size);
        self.depth_texture_view = Self::create_depth_texture();
        self.multisample_texture_view = Self::create_multisample_texture();
    }

    fn update_camera(&mut self) {
        let projection = self.camera2d.build_view_projection_matrix();
        self.sprite_batch.update_projection(projection);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.multisample_texture_view,
                    resolve_target: Some(view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        let clear = encoder.finish();

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let surface = SurfaceInfo {
            format: state.config.format,
            sample_count: 4,
            color_attachment: &self.multisample_texture_view,
            resolve_target: Some(view),
        };

        self.ui_renderer.paint_with_encoder(
            &mut self.ui_yak,
            &state.device,
            &state.queue,
            &mut encoder,
            surface,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.multisample_texture_view,
                    resolve_target: Some(view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
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

            self.sprite_batch.render_pass(&mut render_pass);
        }

        let game_commands = encoder.finish();
        state.queue.submit([clear, game_commands]);
        frame.present();
        Ok(())
    }
}

static mut ROMA: Option<Roma> = None;
static mut STATE: Option<State> = None;

pub(crate) async fn init_roma(window: winit::window::Window, settings: Settings) {
    env_logger::init();

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

pub fn register_texture(path: PathBuf) -> u64 {
    let roma = get_roma();
    roma.sprite_batch.register_texture(path)
}

pub fn draw_image(texture_id: u64, params: DrawImageParams) {
    let roma = get_roma();
    roma.sprite_batch.queue(texture_id, params);
}

pub fn draw_image_multiple(texture_id: u64, params: &mut [DrawImageParams]) {
    let roma = get_roma();
    roma.sprite_batch.queue_multiple(texture_id, params);
}

pub type ParsedText = (Vec<bmfont::CharPosition>, u32);

pub fn parse_text(text: &SmolStr) -> ParsedText {
    let roma = get_roma();
    roma.fonts.parse_text(text)
}

pub fn draw_parsed_text(
    (char_positions, total_width): &ParsedText,
    position: &[f32; 3],
    color: Color,
) {
    let roma = get_roma();
    let offset_x = (*total_width as f32 / 2.).ceil();
    let [draw_x, draw_y, draw_z] = position;
    let draw_x = draw_x - offset_x;

    let chars_staging = &mut roma.staging;
    let chars_len = char_positions.len();

    if chars_staging.len() < chars_len {
        chars_staging.resize(chars_len * 2, DrawImageParams::default());
    }

    for (i, char) in char_positions.iter().enumerate() {
        let x = char.screen_rect.x;
        let y = char.screen_rect.y;
        let source = [
            char.page_rect.x as u16,
            char.page_rect.y as u16,
            char.screen_rect.width as u16,
            char.screen_rect.height as u16,
        ];

        let x = draw_x + x as f32;
        let y = draw_y + y as f32;
        chars_staging[i] = DrawImageParams::new(&[x, y, *draw_z], color, source);
    }
    roma.sprite_batch
        .queue_multiple(TEXT_ATLAS_TEXTURE_ID, &mut chars_staging[..chars_len]);
}

pub fn draw_text(params: DrawTextParams) {
    let roma = get_roma();
    let Some(parsed_text) = roma.fonts.parse(params.text) else {
        return;
    };
    draw_parsed_text(parsed_text, &params.position, params.color)
}

pub fn set_camera_size(width: f32, height: f32) {
    let roma = get_roma();
    roma.camera2d.set_size(width, height);
}

pub fn set_camera_zoom(zoom: camera::Zoom) {
    let roma = get_roma();
    roma.camera2d.set_zoom(zoom);
}

pub fn get_camera_zoom() -> &'static camera::Zoom {
    let roma = get_roma();
    &roma.camera2d.zoom
}
pub fn get_camera_size() -> (f32, f32) {
    let roma = get_roma();
    let camera2d = &roma.camera2d;
    (camera2d.width, camera2d.height)
}

pub fn get_screen_size() -> (usize, usize) {
    let state = get_state();
    (state.size.width as usize, state.size.height as usize)
}

pub fn set_camera_position(x: f32, y: f32) {
    let roma = get_roma();
    roma.camera2d.set_position(x, y);
}

pub fn get_input() -> &'static winit_input_helper::WinitInputHelper {
    let roma = get_roma();
    roma.input()
}

pub fn get_delta() -> Duration {
    let roma = get_roma();
    roma.get_delta()
}

pub fn add_font(name: &str, bytes: &[u8]) {
    let roma = get_roma();
    let fonts = roma
        .ui_yak
        .dom()
        .get_global_or_init(yakui::font::Fonts::default);
    let font = yakui::font::Font::from_bytes(bytes, yakui::font::FontSettings::default()).unwrap();
    fonts.add(font, Some(name));
}

/// This function takes some bytes and turns it into a yakui `Texture` object so
/// that we can reference it later in our UI.
pub fn add_ui_texture(bytes: &[u8], filter: yakui::paint::TextureFilter) -> ManagedTextureId {
    let image = image::load_from_memory(bytes).unwrap().into_rgba8();
    let size = yakui::UVec2::new(image.width(), image.height());

    let mut texture = yakui::paint::Texture::new(
        yakui::paint::TextureFormat::Rgba8Srgb,
        size,
        image.into_raw(),
    );
    texture.mag_filter = filter;

    let roma = get_roma();
    roma.ui_yak.add_texture(texture)
}

pub fn run_game<T: 'static>(
    settings: Settings,
    mut init: impl FnMut() -> T + 'static,
    mut game_loop: impl FnMut(&mut T) + 'static,
    mut resize_handler: impl FnMut(&mut T, (usize, usize)) + 'static,
) {
    use winit::{
        event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    pollster::block_on(async {
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

        let mut state = init();

        let mut last_tick = Instant::now();
        event_loop.run(move |window_event, _, control_flow| {
            get_roma()
                .ui_window
                .handle_event(&mut get_roma().ui_yak, &window_event);

            get_roma().input().update(&window_event);

            match window_event {
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
                        resize_handler(
                            &mut state,
                            (physical_size.width as usize, physical_size.height as usize),
                        );
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        get_roma().resize(new_inner_size);
                        resize_handler(
                            &mut state,
                            (
                                new_inner_size.width as usize,
                                new_inner_size.height as usize,
                            ),
                        );
                    }
                    _ => {
                        get_roma().input().update(&window_event);
                    }
                },
                Event::RedrawEventsCleared => {
                    get_state().window.request_redraw();
                }
                Event::RedrawRequested(window_id) if window_id == this_window_id => {
                    let now = Instant::now();
                    let delta = now - last_tick;
                    last_tick = now;

                    get_roma().set_delta(delta);
                    get_roma().ui_yak.start();
                    game_loop(&mut state);
                    get_roma().ui_yak.finish();

                    match get_roma().render() {
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            let roma = get_roma();
                            let new_size = get_state().size;
                            roma.resize(&new_size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    });
}
