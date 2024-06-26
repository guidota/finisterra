use std::sync::Arc;

use wgpu::PresentMode;

pub(crate) struct State {
    pub(crate) window: Arc<winit::window::Window>,
    pub(crate) size: engine::window::Size,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) limits: wgpu::Limits,
}

impl State {
    pub async fn initialize(window: winit::window::Window, present_mode: PresentMode) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let window = Arc::new(window);
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
            .await
            .expect("[renderer] couldn't request adapter");

        let optional_features = wgpu::Features::DEPTH_CLIP_CONTROL;
        let required_features = wgpu::Features::PUSH_CONSTANTS
            .union(wgpu::Features::TEXTURE_BINDING_ARRAY)
            .union(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING);
        let adapter_features = adapter.features();
        let mut required_limits = adapter.limits();
        required_limits.max_push_constant_size = 64;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: (optional_features & adapter_features) | required_features,
                    required_limits,
                },
                None,
            )
            .await
            .expect("[renderer] couldn't request device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = wgpu::TextureFormat::Rgba8Unorm;

        let present_mode = if surface_caps.present_modes.contains(&present_mode) {
            present_mode
        } else {
            surface_caps.present_modes[0]
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 3,
        };

        surface.configure(&device, &config);

        Self {
            window,
            size: engine::window::Size {
                width: size.width as u16,
                height: size.height as u16,
            },
            config,
            surface,
            device,
            queue,
            limits: adapter.limits(),
        }
    }

    pub fn resize(&mut self, size: engine::window::Size) {
        self.config.width = size.width as u32;
        self.config.height = size.height as u32;
        self.size = size;
        self.surface.configure(&self.device, &self.config);
    }
}
