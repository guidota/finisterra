use std::iter::once;

pub use wgpu::{
    Adapter, Backends, Color, CommandEncoder, CommandEncoderDescriptor, CompositeAlphaMode, Device,
    DeviceDescriptor, Instance, InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference,
    PresentMode::AutoNoVsync, Queue, RenderPass, RenderPassColorAttachment, RequestAdapterOptions,
    Surface, SurfaceConfiguration, SurfaceError, SurfaceTexture, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};
use wgpu::{BindGroup, Buffer, RenderPipeline};
pub use winit::{dpi::PhysicalSize, window::Window};

use super::camera::CameraState;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum LayoutKind {
    Texture,
    Camera,
}

pub struct Renderer {
    surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub camera: CameraState,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let instance = create_instance();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = create_adapter(instance, &surface).await;
        let (device, queue) = create_device_and_queue(&adapter).await;

        let config = create_surface_config(&surface, adapter, window.inner_size(), window);

        surface.configure(&device, &config);
        let camera = CameraState::new(&device);

        Self {
            surface,
            device,
            queue,
            config,
            camera,
        }
    }

    pub fn size(&self) -> PhysicalSize<f32> {
        PhysicalSize::new(self.config.width as f32, self.config.height as f32)
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, scale_factor: f64) {
        self.config.width = new_size.width * scale_factor as u32;
        self.config.height = new_size.height * scale_factor as u32;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn update(&self) {
        self.camera.update_projection(&self.queue, self.size());
    }

    pub fn render(
        &mut self,
        draws: &[(&RenderPipeline, &BindGroup, (&Buffer, &Buffer))],
    ) -> Result<(), SurfaceError> {
        let (output, view, mut encoder) = self.prepare_render()?;

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Roma - Main pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        for draw in draws {
            render_pass.set_pipeline(draw.0);
            render_pass.set_bind_group(0, draw.1, &[]);
            render_pass.set_bind_group(1, &self.camera.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, draw.2 .0.slice(..));
            render_pass.set_index_buffer(draw.2 .1.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..1);
        }
        drop(render_pass);

        self.queue.submit(once(encoder.finish()));
        output.present();
        Ok(())
    }

    fn prepare_render(
        &mut self,
    ) -> Result<(SurfaceTexture, TextureView, CommandEncoder), SurfaceError> {
        let encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        Ok((output, view, encoder))
    }
}

fn create_surface_config(
    surface: &Surface,
    adapter: Adapter,
    size: PhysicalSize<u32>,
    window: &Window,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width: size.width * window.scale_factor() as u32,
        height: size.height * window.scale_factor() as u32,
        present_mode: AutoNoVsync,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![TextureFormat::Bgra8UnormSrgb],
    }
}

fn create_instance() -> Instance {
    Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        dx12_shader_compiler: Default::default(),
    })
}

fn optional_features() -> wgpu::Features {
    wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
}

fn required_features() -> wgpu::Features {
    wgpu::Features::TEXTURE_BINDING_ARRAY
}

async fn create_device_and_queue(adapter: &Adapter) -> (Device, Queue) {
    let features = (optional_features() & adapter.features()) | required_features();
    adapter
        .request_device(
            &DeviceDescriptor {
                features,
                limits: Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap()
}

async fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap()
}
