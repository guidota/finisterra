use std::{cmp::Ordering, collections::HashSet};

use wgpu::{util::StagingBelt, *};
use winit::{window::*, *};

use crate::render::{
    sprite_batch::{prepare_sprite_data, SpriteBatchRenderPass, SpriteData},
    texture_array::{IndexedVertex, TextureArrayRenderPass},
};

use self::{
    camera::{Camera, DeviceCameraExt},
    texture_array::TextureArray,
    textures::Textures,
};

pub mod camera;
pub mod rect;
pub mod texture_array;
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

    staging_belt: StagingBelt,
    frame_draws: Vec<SpriteData>,
    pub(crate) textures: Textures,
    pub(crate) texture_array: TextureArray,
    pub(crate) sprite_batch_pass: SpriteBatchRenderPass,
    pub(crate) texture_array_pass: TextureArrayRenderPass,
}

impl Graphics {
    pub async fn new(window: Window) -> Self {
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

        let limits = Limits {
            max_samplers_per_shader_stage: TextureArrayRenderPass::MAX_TEXTURES,
            max_sampled_textures_per_shader_stage: TextureArrayRenderPass::MAX_TEXTURES,
            ..Default::default()
        };
        let optional_features =
            Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING;
        let required_features = Features::TEXTURE_BINDING_ARRAY;
        let adapter_features = adapter.features();
        let features = (optional_features & adapter_features) | required_features;
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features,
                    limits,
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

        let textures = Textures::new(&device);
        let frame_draws = vec![];

        let sprite_batch_pass = SpriteBatchRenderPass::new(
            &device,
            &config,
            &[&textures.bind_group_layout, &camera.bind_group_layout],
        );

        let texture_array = TextureArray::new(&device, &queue);
        let texture_array_pass =
            TextureArrayRenderPass::new(&device, &config, &camera.bind_group_layout);

        let staging_belt = StagingBelt::new(1024 * 4);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            staging_belt,
            textures,
            texture_array,
            frame_draws,
            sprite_batch_pass,
            texture_array_pass,
            camera,
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

    pub fn push_draw(&mut self, sprite_data: SpriteData) {
        self.frame_draws.push(sprite_data);
    }

    // pub fn render_batched(&mut self) -> Result<(), wgpu::SurfaceError> {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let (vertices, batches) = prepare_sprite_data(&mut self.frame_draws);
        let vertices = bytemuck::cast_slice(&vertices);
        self.queue
            .write_buffer(&self.sprite_batch_pass.vertex_buffer, 0, vertices);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            // render_pass.set_scissor_rect(20, 60, 480, 480);
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

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    // pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    pub fn render_indexed(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // order sprites by z then by texture id
        self.frame_draws
            .sort_unstable_by(|a, b| match a.z.partial_cmp(&b.z) {
                Some(Ordering::Equal) | None => Ordering::Less,
                Some(other) => other,
            });

        let texture_ids = self
            .frame_draws
            .iter()
            .fold(HashSet::new(), |mut set, sprite_data| {
                set.insert(sprite_data.texture_id.clone());
                set
            });
        self.texture_array.update_bind_group(
            texture_ids,
            &self.textures,
            &self.device,
            &self.texture_array_pass.bind_group_layout,
        );

        for draw in &self.frame_draws {
            let index = self.texture_array.get_index(&draw.texture_id);
            let indexed_vertices = draw
                .vertices
                .iter()
                .map(|vertex| IndexedVertex::from(*vertex, index))
                .collect();

            self.texture_array.update_vertex_buffer(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                draw.entity_id,
                indexed_vertices,
            );
        }
        self.staging_belt.finish();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            // render_pass.set_viewport(20., 60., 480., 480., 0., 1.);
            if let Some(bind_group) = &self.texture_array.bind_group {
                render_pass.set_pipeline(&self.texture_array_pass.pipeline);
                render_pass.set_bind_group(0, bind_group, &[0]);
                render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
                render_pass.set_index_buffer(
                    self.texture_array_pass.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                for draw in self.frame_draws.drain(..) {
                    let vertex_buffer = self
                        .texture_array
                        .get_vertex_buffer(draw.entity_id)
                        .unwrap();
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.draw_indexed(0..6, 0, 0..1);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn render_batch2(&mut self) -> Result<(), wgpu::SurfaceError> {
        // pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // order sprites by z then by texture id
        self.frame_draws
            .sort_unstable_by(|a, b| match a.z.partial_cmp(&b.z) {
                Some(Ordering::Equal) | None => Ordering::Less,
                Some(other) => other,
            });

        for draw in &self.frame_draws {
            let index = self.texture_array.get_index(&draw.texture_id);
            let indexed_vertices = draw
                .vertices
                .iter()
                .map(|vertex| IndexedVertex::from(*vertex, index))
                .collect();

            self.texture_array.update_vertex_buffer_2(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                draw.entity_id,
                indexed_vertices,
            );
        }
        self.staging_belt.finish();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            // render_pass.set_viewport(20., 60., 480., 480., 0., 1.);
            render_pass.set_pipeline(&self.sprite_batch_pass.pipeline);
            render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
            render_pass.set_index_buffer(
                self.texture_array_pass.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            let mut last_texture_id = "".to_string();
            for draw in self.frame_draws.drain(..) {
                if draw.texture_id != last_texture_id {
                    let (_, bind_group) = self.textures.get_texture(&draw.texture_id).unwrap();
                    render_pass.set_bind_group(0, bind_group, &[]);
                    last_texture_id = draw.texture_id;
                }
                let vertex_buffer = self
                    .texture_array
                    .get_vertex_buffer(draw.entity_id)
                    .unwrap();
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.draw_indexed(0..6, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
