use std::{ops::Range, num::NonZeroU32};

use engine::{
    camera::Viewport,
    draw::{image::DrawImage, Target},
};
use nohash_hasher::IntMap;
use wgpu::{util::DeviceExt, Device, PushConstantRange, Queue, ShaderStages, SurfaceConfiguration, BindGroup};

use self::texture_array::TextureArray;

mod texture_array;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    pub(crate) bind_group_layout: wgpu::BindGroupLayout,

    draws_to_textures: IntMap<u64, Vec<DrawImage>>,
    draws_to_world: Vec<DrawImage>,
    draws_to_ui: Vec<DrawImage>,

    pub(crate) pre_render_texture_array: TextureArray,
    pub(crate) texture_array: TextureArray,
}

impl Renderer {
    pub fn initialize(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("renderer/shader.wgsl"));

        let bind_group_layout =
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
                        count: NonZeroU32::new(10000),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(10000),
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: wgpu::BindingResource::TextureViewArray(&[]),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: wgpu::BindingResource::SamplerArray(&[]),
        //         },
        //     ],
        //     layout: &bind_group_layout,
        //     label: Some("bind group"),
        // });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
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
                    attributes: &wgpu::vertex_attr_array![0 => Uint32, 1 => Float32, 2 => Unorm8x4, 3 => Uint32x2, 4 => Sint32],
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

        let pre_render_texture_array = TextureArray::new();
        let texture_array = TextureArray::new();
        Self {
            pipeline,
            vertex_buffer,

            bind_group_layout,

            draws_to_textures: IntMap::default(),
            draws_to_world: vec![],
            draws_to_ui: vec![],

            pre_render_texture_array,
            texture_array,
        }
    }

    pub fn draw_image(&mut self, parameters: DrawImage, target: Target) {
        match target {
            Target::World => {
                self.draws_to_world.push(parameters);
                            }
            Target::UI => {
                self.draws_to_ui.push(parameters);
            }
            Target::Texture {
                id: target_texture_id,
            } => {
                self.draws_to_textures
                    .entry(target_texture_id)
                    .or_default()
                    .push(parameters);
            }
        }
    }

    pub fn prepare(&mut self, device: &Device, queue: &Queue) -> Instructions {
        // ensure space in vertex buffer
        let queue_size = self.draws_to_world.len() + self.draws_to_ui.len() + self.draws_to_textures.iter().fold(0, |size, (_, draws)| {
            size + draws.len()
        });
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

        let mut to_textures_ranges = vec![];


        let mut offset = 0;
        for (texture_id, draws) in &mut self.draws_to_textures {
            for draw in draws.iter_mut() {
                if let Some(index) = self.pre_render_texture_array.indices.get(&(draw.index as u64)) {
                    draw.index = *index;
                } else {
                    log::error!("trying to render to_texture bad indexed texture {}", draw.index);
                }
            }

            let data = bytemuck::cast_slice(&draws[..]);
            let buffer_offset = (std::mem::size_of::<DrawImage>() * offset) as u64;
            queue.write_buffer(&self.vertex_buffer, buffer_offset, data);

            to_textures_ranges.push((*texture_id, offset..(offset + draws.len())));

            offset += draws.len();
            draws.clear();
        }
        self.draws_to_textures.clear();

        let world_range = offset..(offset + self.draws_to_world.len());
        for draw in self.draws_to_world.iter_mut() {
            if let Some(index) = self.texture_array.indices.get(&(draw.index as u64)) {
                draw.index = *index;
            } else {
                log::error!("trying to render bad indexed texture {}", draw.index);
            }
        }
        let buffer_offset = (std::mem::size_of::<DrawImage>() * offset) as u64;
        let data = bytemuck::cast_slice(&self.draws_to_world[..]);
        queue.write_buffer(&self.vertex_buffer, buffer_offset, data);
        offset += self.draws_to_world.len();
        self.draws_to_world.clear();

        let ui_range = offset..(offset + self.draws_to_ui.len());
        for draw in self.draws_to_ui.iter_mut() {
            if let Some(index) = self.texture_array.indices.get(&(draw.index as u64)) {
                draw.index = *index;
            } else {
                log::error!("trying to render bad indexed texture {}", draw.index);
            }
        }
        let buffer_offset = (std::mem::size_of::<DrawImage>() * offset) as u64;
        let data = bytemuck::cast_slice(&self.draws_to_ui[..]);
        queue.write_buffer(&self.vertex_buffer, buffer_offset, data);
        self.draws_to_ui.clear();

        self.texture_array.prepare(device, &self.bind_group_layout);
        self.pre_render_texture_array.prepare(device, &self.bind_group_layout);

        Instructions { world_range, ui_range, to_textures_ranges }
    }

    

    pub fn prepare_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>, bind_group: &'pass BindGroup) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, bind_group, &[]);
    }

    pub fn render_range<'pass>(
        &'pass self,
        render_pass: &mut wgpu::RenderPass<'pass>,
        range: Range<usize>,
        viewport: &Viewport,
        projection: [[f32; 4]; 4],
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
        render_pass.draw(0..4, range.start as u32..range.end as u32);
    }
}

#[derive(Debug)]
pub struct Instructions {
    pub world_range: Range<usize>,
    pub ui_range: Range<usize>,
    pub to_textures_ranges: Vec<(u64, Range<usize>)>,
}

