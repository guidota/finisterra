use rustc_hash::FxHashMap;
use wgpu::util::DeviceExt;

use crate::{roma::get_state, DrawImageParams, Rect};

pub(crate) mod texture;

type Texture = (wgpu::BindGroup, (usize, usize));

pub(crate) struct ImageRenderer {
    bind_group_layout: wgpu::BindGroupLayout,
    textures: FxHashMap<usize, Option<Texture>>,

    pipeline: wgpu::RenderPipeline,
    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,

    textures_folder: String,

    queue: FxHashMap<usize, Vec<DrawImageStrictParams>>,
}

#[derive(Default, Debug)]
pub struct DrawImageStrictParams {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sx: f32,
    pub sy: f32,
    pub sw: f32,
    pub sh: f32,
    pub texture_width: f32,
    pub texture_height: f32,
    pub flip_y: bool,
    pub color: [f32; 4],
}

struct Instructions {
    vertices: Vec<Vertex>,
    batches: Vec<Batch>,
}

struct Batch {
    texture_id: usize,
    size: u32,
}

impl ImageRenderer {
    // make this dynamic
    pub const MAX_SPRITES: usize = 20000;
    const MAX_INDICES: usize = Self::MAX_SPRITES * 6;
    const MAX_VERTICES: usize = Self::MAX_SPRITES * 4;
    pub fn init(textures_folder: &str, camera_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let state = get_state();
        let device = &state.device;
        let config = &state.config;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
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
                bind_group_layouts: &[&bind_group_layout, camera_bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
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
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut indices: [u16; Self::MAX_INDICES] = [0u16; Self::MAX_INDICES];
        for i in 0..Self::MAX_SPRITES {
            indices[i * 6] = (i * 4) as u16;
            indices[i * 6 + 1] = (i * 4 + 1) as u16;
            indices[i * 6 + 2] = (i * 4 + 2) as u16;
            indices[i * 6 + 3] = (i * 4) as u16;
            indices[i * 6 + 4] = (i * 4 + 2) as u16;
            indices[i * 6 + 5] = (i * 4 + 3) as u16;
        }
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let vertices = [Vertex::default(); Self::MAX_VERTICES];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Batch Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let textures = FxHashMap::default();

        Self {
            bind_group_layout,
            pipeline,
            index_buffer,
            vertex_buffer,
            queue: FxHashMap::default(),
            textures,
            textures_folder: textures_folder.to_string(),
        }
    }

    pub(crate) fn add_texture(&mut self, id: usize, texture: &texture::Texture) {
        self.textures
            .insert(id, Some(texture.to_bind_group(&self.bind_group_layout)));
    }

    fn load_texture(&mut self, id: usize) {
        if self.textures.contains_key(&id) {
            return;
        }
        let state = get_state();
        let device = &state.device;
        let queue = &state.queue;
        let path = format!("{}/{}.png", self.textures_folder, id);
        let texture = match texture::Texture::from_path(device, queue, &path) {
            Ok(texture) => Some(texture.to_bind_group(&self.bind_group_layout)),
            _ => None,
        };

        self.textures.insert(id, texture);
    }

    pub fn queue(&mut self, params: DrawImageParams) {
        let id = params.texture_id;
        self.load_texture(id);

        // if valid request, convert and push to queue
        if let Some(Some((_, dimensions))) = self.textures.get(&id) {
            let params = params.convert_to_strict(*dimensions);
            self.queue.entry(id).or_default().push(params);
        }
    }

    fn process_queue(&mut self) -> Instructions {
        let mut vertices = vec![];
        let mut batches = vec![];

        for (texture_id, batch_draw_params) in self.queue.iter() {
            batches.push(Batch {
                texture_id: *texture_id,
                size: batch_draw_params.len() as u32,
            });
            for draw_params in batch_draw_params {
                vertices.append(&mut draw_params.into());
            }
        }
        self.queue.clear();
        Instructions { vertices, batches }
    }

    pub fn render_pass<'pass>(
        &'pass mut self,
        render_pass: &mut wgpu::RenderPass<'pass>,
        camera_bind_group: &'pass wgpu::BindGroup,
    ) {
        let instructions = self.process_queue();
        if instructions.batches.is_empty() {
            return;
        }
        let vertices = bytemuck::cast_slice(instructions.vertices.as_slice());
        get_state()
            .queue
            .write_buffer(&self.vertex_buffer, 0, vertices);
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(1, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        let mut offset = 0;
        for Batch { texture_id, size } in instructions.batches {
            if let Some(Some((bind_group, _))) = self.textures.get(&texture_id) {
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.draw_indexed(offset..(offset + size * 6), 0, 0..1);
            }
            offset += size * 6;
        }
    }
}

#[repr(C)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

impl DrawImageParams {
    fn convert_to_strict(self, dimensions: (usize, usize)) -> DrawImageStrictParams {
        let source = self.source.unwrap_or(Rect {
            x: 0,
            y: 0,
            w: dimensions.0,
            h: dimensions.1,
        });
        DrawImageStrictParams {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z,
            sx: source.x as f32,
            sy: source.y as f32,
            sw: source.w as f32,
            sh: source.h as f32,
            texture_width: dimensions.0 as f32,
            texture_height: dimensions.1 as f32,
            flip_y: self.flip_y,
            color: self.color,
        }
    }
}

impl From<&DrawImageStrictParams> for Vec<Vertex> {
    fn from(params: &DrawImageStrictParams) -> Self {
        let texture_width = params.texture_width;
        let texture_height = params.texture_height;
        let flip_y = params.flip_y;
        let x = params.x;
        let y = params.y;
        let z = params.z;
        let sx = params.sx;
        let sy = params.sy;
        let sw = params.sw;
        let sh = params.sh;

        let p = [
            [x, y, z],
            [x + sw, y, z],
            [x + sw, y + sh, z],
            [x, y + sh, z],
        ];

        let mut tex_coords = [
            [sx / texture_width, (sy + sh) / texture_height],
            [(sx + sw) / texture_width, (sy + sh) / texture_height],
            [(sx + sw) / texture_width, sy / texture_height],
            [sx / texture_width, sy / texture_height],
        ];

        if flip_y {
            tex_coords.swap(0, 3);
            tex_coords.swap(1, 2);
        }

        let mut vertices = Vec::with_capacity(4);
        for i in 0..4 {
            let vertex = Vertex {
                position: p[i],
                tex_coords: tex_coords[i],
                color: params.color,
            };
            vertices.push(vertex);
        }
        vertices
    }
}