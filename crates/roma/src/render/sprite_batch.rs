use std::{collections::VecDeque, fmt::Display};

use wgpu::{
    util::DeviceExt, BindGroupLayout, BlendComponent, BlendFactor, BlendOperation, Buffer, Device,
    RenderPipeline, SurfaceConfiguration,
};

use crate::{graphics::vec2::Vec2, resources::texture};

pub(crate) struct SpriteBatchRenderPass {
    pub pipeline: RenderPipeline,
    pub index_buffer: Buffer,
    pub vertex_buffer: Buffer,
}

impl SpriteBatchRenderPass {
    pub const MAX_SPRITES: usize = 2560;
    const MAX_INDICES: usize = Self::MAX_SPRITES * 6;
    const MAX_VERTICES: usize = Self::MAX_SPRITES * 4;

    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        layouts: &[&BindGroupLayout],
    ) -> Self {
        let pipeline = device.create_sprite_pipeline(config, layouts);

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

        Self {
            pipeline,
            index_buffer,
            vertex_buffer,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SpriteData {
    pub z: f32,
    pub texture_id: usize,
    pub entity_id: usize,
}

#[repr(C)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = f.write_str("\n");
        _ = f.write_str(&format!(
            "pos: {:?} tex_coord: {:?}",
            self.position, self.tex_coords
        ));
        f.write_str("\n")
    }
}

impl Vertex {
    pub fn new(position: Vec2, coords: Vec2) -> Vertex {
        Self {
            position: [position.x, position.y, 0.],
            tex_coords: [coords.x, coords.y],
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
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
            ],
        }
    }
}

pub(crate) trait DeviceSpriteBatchPipelineExt {
    fn create_sprite_pipeline(
        &self,
        config: &SurfaceConfiguration,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline;
}

impl DeviceSpriteBatchPipelineExt for Device {
    fn create_sprite_pipeline(
        &self,
        config: &SurfaceConfiguration,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        let shader = self.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout = self.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        self.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                    blend: Some(wgpu::BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::SrcAlpha,
                            operation: BlendOperation::Add,
                        },
                    }),
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
                format: texture::Texture::DEPTH_FORMAT,
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
        })
    }
}

pub struct SpriteBatch {
    pub texture_id: usize,
    pub size: u32,
}

pub fn prepare_sprite_data(sprites: &mut VecDeque<SpriteData>) -> Vec<SpriteBatch> {
    if sprites.is_empty() {
        return vec![];
    }

    let mut batches = vec![];
    let mut current_batch = SpriteBatch {
        size: 0,
        texture_id: sprites[0].texture_id,
    };

    while let Some(sprite) = sprites.pop_back() {
        if sprite.texture_id == current_batch.texture_id {
            current_batch.size += 1;
        } else {
            batches.push(current_batch);
            current_batch = SpriteBatch {
                size: 1,
                texture_id: sprite.texture_id,
            };
        }
    }

    if current_batch.size > 0 {
        batches.push(current_batch);
    }

    batches
}
