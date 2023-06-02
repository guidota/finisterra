use std::{collections::VecDeque, iter::once};

use wgpu::*;
use winit::{window::*, *};

use crate::render::sprite_batch::{prepare_sprite_data, SpriteBatchRenderPass, SpriteData, Vertex};

use self::{
    camera::{Camera, DeviceCameraExt},
    textures::Textures,
};

pub mod camera;
pub mod rect;
pub mod textures;
pub mod vec2;

pub struct Graphics {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: dpi::PhysicalSize<u32>,
    window: Window,
    camera: Camera,

    frame_draws: VecDeque<SpriteData>,
    frame_vertices: Vec<Vertex>,

    pub(crate) textures: Textures,
    pub(crate) sprite_batch_pass: SpriteBatchRenderPass,
}

impl Graphics {
    pub async fn new(window: Window, base_path: String) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: adapter.features(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![surface_format],
        };

        surface.configure(&device, &config);

        let camera = device.create_camera();

        let textures = Textures::new(&device, base_path);

        let frame_draws = VecDeque::new();
        let frame_vertices = vec![];

        let sprite_batch_pass = SpriteBatchRenderPass::new(
            &device,
            &config,
            &[&textures.bind_group_layout, &camera.bind_group_layout],
        );
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            textures,
            frame_draws,
            sprite_batch_pass,
            camera,
            frame_vertices,
        }
    }

    pub(crate) fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn size(&self) -> &dpi::PhysicalSize<u32> {
        &self.size
    }

    pub(crate) fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = *new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn set_camera_projection(&mut self, projection: cgmath::Matrix4<f32>) {
        self.camera.uniform.set_projection(projection);
        self.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform]),
        );
    }

    pub fn push_draw(&mut self, sprite_data: SpriteData, vertices: Vec<Vertex>) {
        self.frame_draws.push_front(sprite_data);
        self.frame_vertices.extend(vertices);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.draw_sprites(&mut encoder, &view);

        self.queue.submit(once(encoder.finish()));
        frame.present();

        self.frame_vertices.clear();

        Ok(())
    }

    fn draw_sprites(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        let batches = prepare_sprite_data(&mut self.frame_draws);

        let vertices = bytemuck::cast_slice(&self.frame_vertices);
        self.queue
            .write_buffer(&self.sprite_batch_pass.vertex_buffer, 0, vertices);

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
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.sprite_batch_pass.pipeline);
            render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.sprite_batch_pass.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.sprite_batch_pass.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            let mut offset = 0;
            for batch in batches {
                if let Some((_, bind_group)) = self.textures.collection.get(&batch.texture_id) {
                    render_pass.set_bind_group(0, bind_group, &[]);
                    render_pass.draw_indexed(offset..(offset + batch.size * 6), 0, 0..1);
                }
                offset += batch.size * 6;
            }
        }
    }
}
