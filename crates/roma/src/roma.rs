use std::{iter::once, time::Duration};

use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::{
    camera::{Camera, Camera2D},
    font::Fonts,
    renderer::ImageRenderer,
    state::State,
    Settings,
};

pub struct Roma {
    input: WinitInputHelper,
    pub(crate) image_renderer: ImageRenderer,

    depth_texture_view: wgpu::TextureView,
    camera: Camera,
    pub(crate) camera2d: Camera2D,
    pub(crate) fonts: Fonts,

    delta: Duration,
}

impl Roma {
    pub async fn new(settings: Settings) -> Self {
        let camera = Camera::init();
        let camera2d = Camera2D::new(settings.width, settings.height);
        let mut image_renderer =
            ImageRenderer::init(&settings.textures_folder, &camera.bind_group_layout);
        let input = WinitInputHelper::new();

        let depth_texture_view = Self::create_depth_texture();
        let (texture_id, font_texture) = Fonts::create_font_texture();
        image_renderer.add_texture(texture_id, &font_texture);

        let fonts = Fonts::init();

        Self {
            input,
            image_renderer,
            camera,
            camera2d,
            depth_texture_view,
            delta: Duration::from_secs(0),
            fonts,
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
        self.camera.update_projection(projection);
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

            self.image_renderer
                .render_pass(&mut render_pass, &self.camera.bind_group);
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

pub(crate) fn get_device() -> &'static wgpu::Device {
    &get_state().device
}
