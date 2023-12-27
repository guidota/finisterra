use std::ops::Range;

use engine::{
    camera::Viewport,
    draw::{image::DrawImage, Target},
};
use nohash_hasher::IntMap;
use wgpu::{util::DeviceExt, Device, PushConstantRange, Queue, ShaderStages, SurfaceConfiguration};

use crate::images::{self, Images};

use self::queue::SpriteQueue;

mod queue;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,

    zero_queue: SpriteQueue,
    queue: SpriteQueue,
    ui_queue: SpriteQueue,
    queue_to_textures: IntMap<u64, SpriteQueue>,
    sprites: Vec<DrawImage>,
}

impl Renderer {
    pub fn initialize(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("renderer/shader.wgsl"));

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[PushConstantRange {
                    stages: ShaderStages::VERTEX,
                    range: 0..64,
                }],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<DrawImage>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![0 => Uint32, 1 => Float32, 2 => Unorm8x4, 3 => Uint32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::COLOR,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: true,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::GreaterEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let sprites = vec![DrawImage::default(); 8912];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Batch Vertex Buffer"),
            contents: bytemuck::cast_slice(&sprites),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            vertex_buffer,

            texture_bind_group_layout,

            zero_queue: SpriteQueue::default(),
            queue: SpriteQueue::default(),
            ui_queue: SpriteQueue::default(),
            queue_to_textures: IntMap::default(),
            sprites,
        }
    }

    pub fn draw_image(&mut self, id: u64, parameters: DrawImage, target: Target) {
        match target {
            Target::World => {
                if parameters.position.z == 0. {
                    self.zero_queue.push(id, parameters);
                } else {
                    self.queue.push(id, parameters);
                }
            }
            Target::UI => {
                self.ui_queue.push(id, parameters);
            }
            Target::Texture {
                id: target_texture_id,
            } => {
                self.queue_to_textures
                    .entry(target_texture_id)
                    .or_default()
                    .push(id, parameters);
            }
        }
    }

    fn update_buffer(&mut self, queue: &Queue, sprites: usize) {
        let sprites_data = bytemuck::cast_slice(&self.sprites[..sprites]);
        queue.write_buffer(&self.vertex_buffer, 0, sprites_data);
    }

    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        images: &mut Images,
    ) -> (Vec<(u64, Vec<Batch>)>, Vec<Batch>, Vec<Batch>) {
        let queue_size = self.zero_queue.size()
            + self.queue.size()
            + self.ui_queue.size()
            + self
                .queue_to_textures
                .iter()
                .fold(0, |mut size, (_, queue)| {
                    size += queue.size();
                    size
                });
        if queue_size == 0 {
            return (vec![], vec![], vec![]);
        }

        if self.sprites.len() < queue_size {
            self.sprites.resize(queue_size, DrawImage::default());
        }

        let queue_size_in_bytes =
            (std::mem::size_of::<DrawImage>() * queue_size) as wgpu::BufferAddress;
        if self.vertex_buffer.size() < queue_size_in_bytes {
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Sprite Batch Vertex Buffer"),
                size: (std::mem::size_of::<DrawImage>() * queue_size * 2) as u64
                    as wgpu::BufferAddress,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        }

        let mut to_texture_batches = vec![];
        let mut world_batches = vec![];
        let mut ui_batches = vec![];

        // load textures if required
        let texture_ids = self
            .zero_queue
            .texture_ids()
            .chain(self.queue.texture_ids())
            .chain(self.ui_queue.texture_ids());

        for texture_id in texture_ids {
            images.load_texture(device, queue, &self.texture_bind_group_layout, *texture_id);
        }
        for texture_ids in self
            .queue_to_textures
            .values()
            .map(|queue| queue.texture_ids())
        {
            for texture_id in texture_ids {
                images.load_texture(device, queue, &self.texture_bind_group_layout, *texture_id);
            }
        }

        let mut offset = 0;

        for (target_texture_id, queue) in self.queue_to_textures.iter_mut() {
            let mut target_batches = vec![];
            for (texture_id, texture_batch) in queue.batches() {
                if texture_batch.is_empty() {
                    continue;
                }

                let batch_start = offset;
                offset += texture_batch.len();
                let batch_end = offset;

                self.sprites[batch_start..batch_end].copy_from_slice(texture_batch.as_slice());

                let batch = Batch {
                    texture_id: *texture_id,
                    range: batch_start as u32..batch_end as u32,
                };
                target_batches.push(batch);
            }
            to_texture_batches.push((*target_texture_id, target_batches));
        }

        for (texture_id, texture_batch) in self.zero_queue.batches().chain(self.queue.batches()) {
            if texture_batch.is_empty() {
                continue;
            }

            let batch_start = offset;
            offset += texture_batch.len();
            let batch_end = offset;

            self.sprites[batch_start..batch_end].copy_from_slice(texture_batch.as_slice());

            let batch = Batch {
                texture_id: *texture_id,
                range: batch_start as u32..batch_end as u32,
            };

            world_batches.push(batch);
        }

        for (texture_id, texture_batch) in self.ui_queue.batches() {
            if texture_batch.is_empty() {
                continue;
            }

            let batch_start = offset;
            offset += texture_batch.len();
            let batch_end = offset;

            self.sprites[batch_start..batch_end].copy_from_slice(texture_batch.as_slice());

            let batch = Batch {
                texture_id: *texture_id,
                range: batch_start as u32..batch_end as u32,
            };

            ui_batches.push(batch);
        }

        self.queue_to_textures.clear();
        self.ui_queue.reset();
        self.zero_queue.reset();
        self.queue.reset();

        self.update_buffer(queue, offset);

        (to_texture_batches, world_batches, ui_batches)
    }

    pub fn prepare_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    }

    pub fn render_batches<'pass>(
        &'pass self,
        render_pass: &mut wgpu::RenderPass<'pass>,
        batches: Vec<Batch>,
        viewport: &Viewport,
        projection: [[f32; 4]; 4],
        textures: &'pass IntMap<u64, Option<images::Texture>>,
    ) {
        render_pass.set_push_constants(ShaderStages::VERTEX, 0, bytemuck::cast_slice(&projection));
        render_pass.set_viewport(
            viewport.x,
            viewport.y,
            viewport.width,
            viewport.height,
            0.,
            1.,
        );
        for Batch { texture_id, range } in batches {
            if let Some(Some(texture)) = textures.get(&texture_id) {
                render_pass.set_bind_group(0, &texture.bind_group, &[]);
                render_pass.draw(0..4, range);
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Batch {
    texture_id: u64,
    range: Range<u32>,
}
