use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Range,
};

use engine::{
    draw::{image::DrawImage, Target},
    engine::TextureID,
    window::Size,
};
use nohash_hasher::IntMap;
use wgpu::{util::DeviceExt, Device, PushConstantRange, Queue, ShaderStages};

use crate::{
    camera::Camera,
    state::State,
    texture::{self, Texture},
    Renderer,
};

use super::textures::Textures;

pub struct SpriteBatchRenderer {
    textures: Textures,

    bind_groups: IntMap<TextureID, wgpu::BindGroup>,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    draws_counter: usize,
    draws_to_textures: IntMap<TextureID, IntMap<TextureID, Vec<DrawImage>>>,
    draws_to_zero_world: IntMap<TextureID, Vec<DrawImage>>,
    transparent_draws_to_world: IntMap<TextureID, Vec<DrawImage>>,
    draws_to_world: IntMap<TextureID, Vec<DrawImage>>,
    draws_to_ui: IntMap<TextureID, Vec<DrawImage>>,

    depth_texture_view: wgpu::TextureView,
    depth_textures: HashMap<engine::window::Size, wgpu::TextureView>,
}

impl Renderer for SpriteBatchRenderer {
    fn resize(&mut self, state: &State, size: Size) {
        self.depth_texture_view = create_depth_texture(state, size);
    }

    fn ensure_texture(&mut self, state: &State, id: TextureID, target: Target) -> bool {
        if self.textures.load_texture(&state.device, &state.queue, id) {
            if let Entry::Vacant(e) = self.bind_groups.entry(id) {
                if let Some(texture) = self.textures.get(id).flatten() {
                    let bind_group = create_bind_group(state, &self.bind_group_layout, texture);
                    e.insert(bind_group);
                }
            }
            if let Target::Texture { id } = target {
                if let Entry::Vacant(e) = self.bind_groups.entry(id) {
                    if let Some(texture) = self.textures.get(id).flatten() {
                        let bind_group = create_bind_group(state, &self.bind_group_layout, texture);
                        e.insert(bind_group);
                    }
                }
            }

            true
        } else {
            false
        }
    }

    fn push_draw_image(&mut self, draw: DrawImage, target: Target) {
        self.draws_counter += 1;
        match target {
            Target::World => {
                if draw.position.z == 0.0 {
                    self.draws_to_zero_world
                        .entry(draw.index)
                        .or_default()
                        .push(draw);
                } else if draw.color[3] < 255 {
                    self.transparent_draws_to_world
                        .entry(draw.index)
                        .or_default()
                        .push(draw);
                } else {
                    self.draws_to_world
                        .entry(draw.index)
                        .or_default()
                        .push(draw);
                }
            }
            Target::UI => {
                self.draws_to_ui.entry(draw.index).or_default().push(draw);
            }
            Target::Texture {
                id: target_texture_id,
            } => {
                self.draws_to_textures
                    .entry(target_texture_id)
                    .or_default()
                    .entry(draw.index)
                    .or_default()
                    .push(draw);
            }
        }
    }

    fn render(&mut self, state: &State, world_camera: &Camera, ui_camera: &Camera) {
        let Ok(frame) = state.surface.get_current_texture() else {
            log::error!("");
            return;
        };
        let Instructions {
            to_textures_ranges,
            world_ranges,
            ui_ranges,
        } = self.prepare(&state.device, &state.queue);

        let clear_store_ops = wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: wgpu::StoreOp::Store,
        };

        let depth_clear_store_ops = wgpu::Operations {
            load: wgpu::LoadOp::Clear(0.0),
            store: wgpu::StoreOp::Store,
        };

        let mut commands = vec![];
        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        for (texture_id, batches) in to_textures_ranges {
            if let Some(Some(texture)) = self.textures.get(texture_id) {
                let size = engine::window::Size {
                    width: texture.width,
                    height: texture.height,
                };
                let depth_texture_view = self
                    .depth_textures
                    .entry(size)
                    .or_insert_with(|| create_depth_texture(state, size));

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render To Texture Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture.view,
                        resolve_target: None,
                        ops: clear_store_ops,
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth_texture_view,
                        depth_ops: Some(depth_clear_store_ops),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                let dimensions = Size {
                    width: size.width,
                    height: size.height,
                };
                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                let target_camera = Camera::initialize(dimensions, false);
                Self::use_camera(&mut render_pass, &target_camera);

                for (texture, range) in batches {
                    if let Some(texture_bind_group) = self.bind_groups.get(&texture) {
                        render_pass.set_bind_group(0, texture_bind_group, &[]);
                        render_pass.draw(0..4, range.start as u32..range.end as u32);
                    }
                }
            }
        }
        commands.push(encoder.finish());

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
                    ops: clear_store_ops,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(depth_clear_store_ops),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            Self::use_camera(&mut render_pass, world_camera);
            for (texture_id, range) in world_ranges {
                if let Some(texture_bind_group) = self.bind_groups.get(&texture_id) {
                    render_pass.set_bind_group(0, texture_bind_group, &[]);
                    render_pass.draw(0..4, range.start as u32..range.end as u32);
                }
            }

            Self::use_camera(&mut render_pass, ui_camera);
            for (texture_id, range) in ui_ranges {
                if let Some(texture_bind_group) = self.bind_groups.get(&texture_id) {
                    render_pass.set_bind_group(0, texture_bind_group, &[]);
                    render_pass.draw(0..4, range.start as u32..range.end as u32);
                }
            }
        }
        commands.push(encoder.finish());

        state.queue.submit(commands);
        frame.present();
    }

    fn add_texture_file(&mut self, path: &str) -> TextureID {
        self.textures.add_file(path)
    }

    fn set_texture_file(&mut self, path: &str, id: TextureID) {
        self.textures.set_file(id, path);
    }

    fn add_texture(&mut self, texture: Texture) -> TextureID {
        self.textures.add_texture(texture)
    }

    fn texture_dimensions(&mut self, texture_id: TextureID) -> Option<(u16, u16)> {
        self.textures
            .get(texture_id)
            .flatten()
            .map(|texture| (texture.width, texture.height))
    }
}

impl SpriteBatchRenderer {
    pub fn initialize(state: &State) -> Self {
        let size = engine::window::Size {
            width: state.config.width as u16,
            height: state.config.height as u16,
        };
        let depth_texture_view = create_depth_texture(state, size);
        let bind_group_layout = create_bind_group_layout(state);
        let pipeline = create_pipeline(state, &bind_group_layout);
        let sprites: Vec<DrawImage> = vec![];
        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Sprite Batch Renderer Vertex Buffer"),
                contents: bytemuck::cast_slice(&sprites),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        Self {
            textures: Textures::initialize(),
            bind_groups: IntMap::default(),
            bind_group_layout,
            pipeline,
            vertex_buffer,

            draws_counter: 0,
            draws_to_textures: IntMap::default(),
            draws_to_zero_world: IntMap::default(),
            transparent_draws_to_world: IntMap::default(),
            draws_to_world: IntMap::default(),
            draws_to_ui: IntMap::default(),

            depth_texture_view,
            depth_textures: HashMap::default(),
        }
    }

    fn prepare(&mut self, device: &Device, queue: &Queue) -> Instructions {
        self.ensure_vertex_buffer_size(device);

        let mut offset = 0;
        let write_buffer = |draws: &[DrawImage], offset: usize| {
            let data = bytemuck::cast_slice(draws);
            let buffer_offset = (std::mem::size_of::<DrawImage>() * offset) as u64;
            queue.write_buffer(&self.vertex_buffer, buffer_offset, data);
        };

        let prepare_draws = |batches: &mut IntMap<TextureID, Vec<DrawImage>>,
                             ranges: &mut Vec<(TextureID, Range<usize>)>,
                             offset: &mut usize| {
            for (texture_id, draws) in batches.iter_mut() {
                if draws.is_empty() {
                    continue;
                }
                write_buffer(draws, *offset);

                ranges.push((*texture_id, *offset..(*offset + draws.len())));
                *offset += draws.len();
                draws.clear();
            }
        };

        let mut world_ranges = vec![];
        prepare_draws(
            &mut self.draws_to_zero_world,
            &mut world_ranges,
            &mut offset,
        );
        prepare_draws(&mut self.draws_to_world, &mut world_ranges, &mut offset);
        prepare_draws(
            &mut self.transparent_draws_to_world,
            &mut world_ranges,
            &mut offset,
        );

        let mut ui_ranges = vec![];
        prepare_draws(&mut self.draws_to_ui, &mut ui_ranges, &mut offset);

        let mut to_textures_ranges = vec![];
        for (texture, batches) in self.draws_to_textures.iter_mut() {
            let mut current = vec![];
            prepare_draws(batches, &mut current, &mut offset);
            to_textures_ranges.push((*texture, current));
        }
        self.draws_to_textures.clear();

        self.draws_counter = 0;

        Instructions {
            world_ranges,
            ui_ranges,
            to_textures_ranges,
        }
    }

    fn ensure_vertex_buffer_size(&mut self, device: &Device) {
        let required_buffer_size = (std::mem::size_of::<DrawImage>() * self.draws_counter) as u64;
        if self.vertex_buffer.size() < required_buffer_size {
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Sprite Vertex Buffer"),
                size: (std::mem::size_of::<DrawImage>() * (self.draws_counter + 10)) as u64
                    as wgpu::BufferAddress,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        }
    }

    fn use_camera(render_pass: &mut wgpu::RenderPass, camera: &Camera) {
        let viewport = camera.viewport;
        let projection = camera.build_view_projection_matrix();

        render_pass.set_push_constants(ShaderStages::VERTEX, 0, bytemuck::cast_slice(&projection));
        render_pass.set_viewport(
            viewport.x,
            viewport.y,
            viewport.width,
            viewport.height,
            0.,
            1.,
        );
    }
}

fn create_bind_group(
    state: &State,
    bind_group_layout: &wgpu::BindGroupLayout,
    texture: &texture::Texture,
) -> wgpu::BindGroup {
    state.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}

fn create_bind_group_layout(state: &State) -> wgpu::BindGroupLayout {
    state
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        })
}

fn create_pipeline(
    state: &State,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = state
        .device
        .create_shader_module(wgpu::include_wgsl!("sprite_batch_renderer_shader.wgsl"));
    let render_pipeline_layout =
        state
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[PushConstantRange {
                    stages: ShaderStages::VERTEX,
                    range: 0..64,
                }],
            });

    state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                format: state.config.format,
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
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

fn create_depth_texture(state: &State, size: engine::window::Size) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: size.width as u32,
        height: size.height as u32,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[wgpu::TextureFormat::Depth32Float],
    };
    let texture = state.device.create_texture(&desc);

    texture.create_view(&wgpu::TextureViewDescriptor {
        aspect: wgpu::TextureAspect::DepthOnly,
        ..Default::default()
    })
}

type Batches = Vec<(TextureID, Range<usize>)>;

#[derive(Debug)]
pub struct Instructions {
    pub world_ranges: Batches,
    pub ui_ranges: Batches,
    pub to_textures_ranges: Vec<(TextureID, Batches)>,
}
