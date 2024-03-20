use std::{collections::HashMap, num::NonZeroU32, ops::Range};

use engine::{
    draw::{image::DrawImage, Target},
    engine::TextureID,
    window::Size,
};
use nohash_hasher::IntMap;
use wgpu::{util::DeviceExt, Device, PushConstantRange, Queue, ShaderStages, SurfaceConfiguration};

use crate::{camera::Camera, state::State, texture::Texture, Renderer};

use super::{texture_array::TextureArray, textures::Textures};

pub struct TextureArrayRenderer {
    textures: Textures,
    offscreen: Node,
    main: Node,

    draws_to_textures: IntMap<TextureID, Vec<DrawImage>>,
    draws_to_zero_world: Vec<DrawImage>,
    draws_to_world: Vec<DrawImage>,
    draws_to_roof_world: Vec<DrawImage>,
    draws_to_ui: Vec<DrawImage>,

    depth_texture_view: wgpu::TextureView,
    depth_textures: HashMap<engine::window::Size, wgpu::TextureView>,
}

pub struct Node {
    vertex_buffer: wgpu::Buffer,

    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,

    textures_count: u32,
    pub texture_array: TextureArray,
}

impl Renderer for TextureArrayRenderer {
    fn resize(&mut self, state: &State, size: Size) {
        self.depth_texture_view = create_depth_texture(state, size);
    }

    fn ensure_texture(&mut self, state: &State, id: TextureID, target: Target) -> bool {
        if self.textures.load_texture(&state.device, &state.queue, id) {
            let texture_array = match target {
                Target::World | Target::UI => &mut self.main.texture_array,
                _ => &mut self.offscreen.texture_array,
            };
            if !texture_array.has_texture(id) {
                let texture = self.textures.get(id).unwrap().unwrap();
                let view = texture.view.clone();
                let sampler = texture.sampler.clone();

                texture_array.push(id, view, sampler);
            }
            return true;
        }
        false
    }

    fn push_draw_image(&mut self, draw: DrawImage, target: Target) {
        match target {
            Target::World => {
                if draw.position.z == 0.0 {
                    self.draws_to_zero_world.push(draw);
                } else if draw.position.z == 0.99 {
                    self.draws_to_roof_world.push(draw);
                } else {
                    self.draws_to_world.push(draw);
                }
            }
            Target::UI => {
                self.draws_to_ui.push(draw);
            }
            Target::Texture {
                id: target_texture_id,
            } => {
                self.draws_to_textures
                    .entry(target_texture_id)
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
            world_range,
            ui_range,
        } = self.prepare(&state.device, &state.queue, &state.config);

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

        for (texture_id, range) in to_textures_ranges {
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
                let target_camera = Camera::initialize(dimensions, true);
                self.offscreen.prepare_pass(&mut render_pass);
                Self::render_range(&mut render_pass, range, &target_camera);
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

            self.main.prepare_pass(&mut render_pass);
            Self::render_range(&mut render_pass, world_range, world_camera);
            Self::render_range(&mut render_pass, ui_range, ui_camera);
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

impl TextureArrayRenderer {
    pub fn initialize(state: &State) -> Self {
        let size = engine::window::Size {
            width: state.config.width as u16,
            height: state.config.height as u16,
        };
        let depth_texture_view = create_depth_texture(state, size);

        Self {
            textures: Textures::initialize(),

            draws_to_textures: IntMap::default(),
            draws_to_zero_world: vec![],
            draws_to_world: vec![],
            draws_to_roof_world: vec![],
            draws_to_ui: vec![],

            offscreen: Node::initialize(&state.device, &state.config, 1),
            main: Node::initialize(&state.device, &state.config, 1),

            depth_texture_view,
            depth_textures: HashMap::default(),
        }
    }

    fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        config: &SurfaceConfiguration,
    ) -> Instructions {
        let offscreen_draws_len = self
            .draws_to_textures
            .iter()
            .fold(0, |size, (_, draws)| size + draws.len());
        self.offscreen
            .ensure_buffer_size(device, offscreen_draws_len);

        let mut to_textures_ranges = vec![];

        let mut offset = 0;
        for (texture_id, draws) in &mut self.draws_to_textures {
            self.offscreen.update_draws(draws);
            self.offscreen.write_buffer(queue, draws, offset);

            to_textures_ranges.push((*texture_id, offset..(offset + draws.len())));

            offset += draws.len();
            draws.clear();
        }
        self.offscreen.prepare(device, config);

        let main_draws_len = self.draws_to_zero_world.len()
            + self.draws_to_world.len()
            + self.draws_to_roof_world.len()
            + self.draws_to_ui.len();
        self.main.ensure_buffer_size(device, main_draws_len);
        self.main.update_draws(&mut self.draws_to_zero_world);
        self.main.update_draws(&mut self.draws_to_world);
        self.main.update_draws(&mut self.draws_to_roof_world);
        self.main.update_draws(&mut self.draws_to_ui);

        let world_draws = self.draws_to_zero_world.len()
            + self.draws_to_world.len()
            + self.draws_to_roof_world.len();
        let world_range = 0..world_draws;
        let ui_range = world_draws..(world_draws + self.draws_to_ui.len());

        self.main
            .write_buffer(queue, &self.draws_to_zero_world[..], 0);
        self.main.write_buffer(
            queue,
            &self.draws_to_world[..],
            self.draws_to_zero_world.len(),
        );
        self.main.write_buffer(
            queue,
            &self.draws_to_roof_world[..],
            self.draws_to_zero_world.len() + self.draws_to_world.len(),
        );
        self.main.write_buffer(
            queue,
            &self.draws_to_ui[..],
            self.draws_to_zero_world.len()
                + self.draws_to_world.len()
                + self.draws_to_roof_world.len(),
        );

        self.draws_to_textures.clear();
        self.draws_to_zero_world.clear();
        self.draws_to_world.clear();
        self.draws_to_roof_world.clear();
        self.draws_to_ui.clear();

        self.main.prepare(device, config);

        Instructions {
            world_range,
            ui_range,
            to_textures_ranges,
        }
    }

    fn render_range(render_pass: &mut wgpu::RenderPass, range: Range<usize>, camera: &Camera) {
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
        render_pass.draw(0..4, range.start as u32..range.end as u32);
    }
}

impl Node {
    fn initialize(device: &Device, config: &SurfaceConfiguration, textures_count: u32) -> Self {
        let bind_group_layout = create_bind_group_layout(device, textures_count);
        let pipeline = create_pipeline(device, &bind_group_layout, config);

        let sprites = vec![DrawImage::default(); textures_count as usize];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Array Renderer Vertex Buffer"),
            contents: bytemuck::cast_slice(&sprites),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let texture_array = TextureArray::new();
        Self {
            vertex_buffer,
            bind_group_layout,
            pipeline,
            texture_array,
            textures_count,
        }
    }

    fn ensure_buffer_size(&mut self, device: &Device, size: usize) {
        let queue_size_in_bytes = (std::mem::size_of::<DrawImage>() * size) as wgpu::BufferAddress;
        if self.vertex_buffer.size() < queue_size_in_bytes {
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Sprite Vertex Buffer"),
                size: (std::mem::size_of::<DrawImage>() * (size + 10)) as u64
                    as wgpu::BufferAddress,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        }
    }

    fn write_buffer(&mut self, queue: &Queue, draws: &[DrawImage], offset: usize) {
        let data = bytemuck::cast_slice(draws);
        let buffer_offset = (std::mem::size_of::<DrawImage>() * offset) as u64;
        queue.write_buffer(&self.vertex_buffer, buffer_offset, data);
    }

    fn update_draws(&self, draws: &mut [DrawImage]) {
        for draw in draws.iter_mut() {
            if let Some(index) = self.texture_array.get_index(draw.index as TextureID) {
                draw.index = index;
            }
        }
    }

    fn prepare(&mut self, device: &Device, config: &SurfaceConfiguration) {
        let size = self.texture_array.size();
        if size > self.textures_count {
            self.bind_group_layout = create_bind_group_layout(device, size);
            self.pipeline = create_pipeline(device, &self.bind_group_layout, config);
            self.textures_count = size;
        }

        self.texture_array.prepare(device, &self.bind_group_layout);
    }

    fn prepare_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        if let Some(bind_group) = self.texture_array.get_bind_group() {
            render_pass.set_bind_group(0, bind_group, &[]);
        }
    }
}
fn create_pipeline(
    device: &Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    config: &SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    let shader =
        device.create_shader_module(wgpu::include_wgsl!("texture_array_renderer_shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[PushConstantRange {
            stages: ShaderStages::VERTEX,
            range: 0..64,
        }],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
    })
}

fn create_bind_group_layout(device: &Device, count: u32) -> wgpu::BindGroupLayout {
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
                count: NonZeroU32::new(count),
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: NonZeroU32::new(count),
            },
        ],
        label: Some("texture_bind_group_layout"),
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
        format: wgpu::TextureFormat::Depth24PlusStencil8,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[wgpu::TextureFormat::Depth24PlusStencil8],
    };
    let texture = state.device.create_texture(&desc);

    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

#[derive(Debug)]
pub struct Instructions {
    pub world_range: Range<usize>,
    pub ui_range: Range<usize>,
    pub to_textures_ranges: Vec<(TextureID, Range<usize>)>,
}
