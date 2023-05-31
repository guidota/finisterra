use std::{
    fmt::Display,
    num::{NonZeroU32, NonZeroU64},
};

use wgpu::{
    util::DeviceExt, BindGroupLayout, Buffer, Device, RenderPipeline, SurfaceConfiguration,
};

use super::sprite_batch::Vertex;

pub(crate) struct TextureArrayRenderPass {
    pub pipeline: RenderPipeline,
    pub bind_group_layout: BindGroupLayout,
    pub index_buffer: Buffer,
}

impl TextureArrayRenderPass {
    pub const MAX_TEXTURES: u32 = 1024;

    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let bind_group_layout =
            device.create_texture_arrays_bind_group_layout(device, Self::MAX_TEXTURES);

        let pipeline = device.create_texture_arrays_pipeline(
            config,
            device,
            &[&bind_group_layout, camera_bind_group_layout],
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            pipeline,
            bind_group_layout,
            index_buffer,
        }
    }
}

trait DeviceTextureArraysExt {
    fn create_texture_arrays_bind_group_layout(
        &self,
        device: &Device,
        size: u32,
    ) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Array Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // Binding Array
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: NonZeroU32::new(size),
                },
                wgpu::BindGroupLayoutEntry {
                    // Sampler Array
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: NonZeroU32::new(size),
                },
                wgpu::BindGroupLayoutEntry {
                    // Index Uniforms
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(NonZeroU64::new(4).unwrap()),
                    },
                    count: None,
                },
            ],
        })
    }

    fn create_texture_arrays_pipeline(
        &self,
        config: &SurfaceConfiguration,
        device: &Device,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        let mut uniform_workaround = false;
        let base_shader_module =
            device.create_shader_module(wgpu::include_wgsl!("shaders/indexing.wgsl"));
        let env_override = match std::env::var("WGPU_TEXTURE_ARRAY_STYLE") {
            Ok(value) => match &*value.to_lowercase() {
                "nonuniform" | "non_uniform" => Some(true),
                "uniform" => Some(false),
                _ => None,
            },
            Err(_) => None,
        };
        let fragment_entry_point = match (device.features(), env_override) {
            (_, Some(false)) => {
                uniform_workaround = true;
                "uniform_main"
            }
            (_, Some(true)) => "non_uniform_main",
            (f, _)
                if f.contains(
                    wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                ) =>
            {
                "non_uniform_main"
            }
            _ => {
                uniform_workaround = true;
                "uniform_main"
            }
        };
        println!(
            "> create_texture_arrays_pipeline > using uniform workaround {uniform_workaround}"
        );
        let non_uniform_shader_module;
        let fragment_shader_module = if !uniform_workaround {
            non_uniform_shader_module = device
                .create_shader_module(wgpu::include_wgsl!("shaders/non_uniform_indexing.wgsl"));
            &non_uniform_shader_module
        } else {
            &base_shader_module
        };
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("main"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        let vertex_size = std::mem::size_of::<IndexedVertex>();
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &base_shader_module,
                entry_point: "vert_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: vertex_size as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Sint32],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader_module,
                entry_point: fragment_entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}

impl DeviceTextureArraysExt for Device {}

#[repr(C)]
#[derive(PartialEq, PartialOrd, Debug, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IndexedVertex {
    pub pos: [f32; 2],
    pub tex_coord: [f32; 2],
    pub index: u32,
}

impl Display for IndexedVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = f.write_str("\n");
        _ = f.write_str(&format!(
            "pos: {:?} tex_coord: {:?}, index {}",
            self.pos, self.tex_coord, self.index
        ));
        f.write_str("\n")
    }
}

impl IndexedVertex {
    pub fn from(vertex: Vertex, index: u32) -> Self {
        Self {
            pos: [vertex.position[0], vertex.position[1]],
            tex_coord: vertex.tex_coords,
            index,
        }
    }
}
