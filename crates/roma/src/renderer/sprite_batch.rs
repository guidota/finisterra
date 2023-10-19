use nohash_hasher::IntMap;
use std::{collections::HashMap, ops::Range};

use cgmath::ortho;
use wgpu::util::DeviceExt;

use crate::{get_camera_size, get_screen_size, roma::get_state, DrawImageParams};

use super::{queue::SpriteQueue, texture};

type Texture = (wgpu::BindGroup, (usize, usize));

pub(crate) struct SpriteBatch {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    textures: IntMap<u64, Option<Texture>>,
    textures_folder: String,
    texture_bind_group_layout: wgpu::BindGroupLayout,

    zero_queue: SpriteQueue,
    queue: SpriteQueue,
    sprites: Vec<DrawImageParams>,
}

#[derive(Default, Debug)]
struct Batch {
    texture_id: u64,
    range: Range<u32>,
}

impl SpriteBatch {
    pub const INITIAL_SPRITES: usize = 8192;
    pub fn init(textures_folder: &str) -> Self {
        let state = get_state();
        let device = &state.device;
        let config = &state.config;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let projection: [[f32; 4]; 4] =
            ortho(0., 0., config.width as f32, config.height as f32, -1., 1.).into();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&projection),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

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
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<DrawImageParams>() as wgpu::BufferAddress,
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
                    write_mask: wgpu::ColorWrites::ALL,
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
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let sprites = vec![DrawImageParams::default(); Self::INITIAL_SPRITES];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Batch Vertex Buffer"),
            contents: bytemuck::cast_slice(&sprites),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            vertex_buffer,

            camera_buffer,
            camera_bind_group,

            texture_bind_group_layout,
            textures: HashMap::default(),
            textures_folder: textures_folder.to_string(),

            zero_queue: SpriteQueue::default(),
            queue: SpriteQueue::default(),
            sprites,
        }
    }

    pub(crate) fn add_texture(&mut self, id: u64, texture: &texture::Texture) {
        self.textures.insert(
            id,
            Some(texture.to_bind_group(&self.texture_bind_group_layout)),
        );
    }

    fn load_texture(&mut self, id: &u64) -> bool {
        if self.textures.contains_key(id) {
            return true;
        }
        let state = get_state();
        let device = &state.device;
        let queue = &state.queue;
        let path = format!("{}/{}.png", self.textures_folder, id);
        let texture = match texture::Texture::from_path(device, queue, &path) {
            Ok(texture) => Some(texture.to_bind_group(&self.texture_bind_group_layout)),
            _ => {
                println!("Texture not found on {path}");
                None
            }
        };

        let exists = texture.is_some();
        self.textures.insert(*id, texture);

        exists
    }

    pub fn update_projection(&mut self, projection: [[f32; 4]; 4]) {
        let state = get_state();
        state
            .queue
            .write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&projection));
    }

    pub fn queue(&mut self, texture_id: u64, params: DrawImageParams) {
        if params.z == 0.0 {
            self.zero_queue.push(texture_id, params);
        } else {
            self.queue.push(texture_id, params);
        }
    }

    pub fn queue_multiple(&mut self, texture_id: u64, params: &mut [DrawImageParams]) {
        if params.is_empty() {
            return;
        }
        if params[0].z == 0.0 {
            self.zero_queue.push_all(texture_id, params);
        } else {
            self.queue.push_all(texture_id, params);
        }
    }

    fn process_queue(&mut self) -> Vec<Batch> {
        let queue_size = self.zero_queue.size() + self.queue.size();
        if queue_size == 0 {
            return vec![];
        }

        if self.sprites.len() < queue_size {
            self.sprites.resize(queue_size, DrawImageParams::default());
        }

        let queue_size_in_bytes =
            (std::mem::size_of::<DrawImageParams>() * queue_size) as wgpu::BufferAddress;
        if self.vertex_buffer.size() < queue_size_in_bytes {
            let device = &get_state().device;
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Sprite Batch Vertex Buffer"),
                size: (std::mem::size_of::<DrawImageParams>() * queue_size * 2) as u64
                    as wgpu::BufferAddress,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        }

        let mut batches = vec![];

        // load textures if required
        let mut texture_ids = self.zero_queue.texture_ids();
        texture_ids.append(&mut self.queue.texture_ids());

        for texture_id in &texture_ids {
            self.load_texture(texture_id);
        }

        // let mut batch_offset = 0;
        let mut offset = 0;

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

            batches.push(batch);
        }

        self.zero_queue.reset();
        self.queue.reset();

        self.update_buffer(offset);

        batches
    }

    fn update_buffer(&mut self, sprites: usize) {
        let sprites_data = bytemuck::cast_slice(&self.sprites[..sprites]);
        get_state()
            .queue
            .write_buffer(&self.vertex_buffer, 0, sprites_data);
    }

    pub fn render_pass<'pass>(&'pass mut self, render_pass: &mut wgpu::RenderPass<'pass>) {
        let batches = self.process_queue();
        if batches.is_empty() {
            return;
        }

        let (camera_width, camera_height) = get_camera_size();
        let (_, screen_height) = get_screen_size();
        let border = 10.;

        render_pass.set_viewport(
            border,
            screen_height as f32 - camera_height - border,
            camera_width,
            camera_height,
            0.,
            1.,
        );
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

        for Batch { texture_id, range } in batches {
            if let Some(Some((bind_group, _))) = self.textures.get(&texture_id) {
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.draw(0..4, range);
            }
        }
    }
}
