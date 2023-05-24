use std::num::{NonZeroU32, NonZeroU64};

use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Device, RenderPipeline, Sampler, ShaderModule,
    SurfaceConfiguration, TextureView,
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    _pos: [f32; 2],
    _tex_coord: [f32; 2],
    _index: u32,
}

pub fn vertex(pos: [i8; 2], tc: [i8; 2], index: i8) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
        _index: index as u32,
    }
}

pub fn create_vertices() -> Vec<Vertex> {
    vec![
        // left rectangle
        vertex([-1, -1], [0, 1], 0),
        vertex([-1, 1], [0, 0], 0),
        vertex([0, 1], [1, 0], 0),
        vertex([0, -1], [1, 1], 0),
        // right rectangle
        vertex([0, -1], [0, 1], 1),
        vertex([0, 1], [0, 0], 1),
        vertex([1, 1], [1, 0], 1),
        vertex([1, -1], [1, 1], 1),
    ]
}

pub fn create_indices() -> Vec<u16> {
    vec![
        // Left rectangle
        0, 1, 2, // 1st
        2, 0, 3, // 2nd
        // Right rectangle
        4, 5, 6, // 1st
        6, 4, 7, // 2nd
    ]
}

pub fn create_shader(device: &Device) -> (ShaderModule, &str, Option<ShaderModule>) {
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
    let non_uniform_shader_module;
    // TODO: Because naga's capibilities are evaluated on validate, not on write, we cannot make a shader module with unsupported
    // capabilities even if we don't use it. So for now put it in a separate module.
    let fragment_shader_module = if !uniform_workaround {
        non_uniform_shader_module =
            device.create_shader_module(wgpu::include_wgsl!("shaders/non_uniform_indexing.wgsl"));
        Some(non_uniform_shader_module)
    } else {
        None
    };
    (
        base_shader_module,
        fragment_entry_point,
        fragment_shader_module,
    )
}

pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: NonZeroU32::new(2),
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: NonZeroU32::new(2),
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: NonZeroU32::new(2),
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
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

pub fn create_render_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
    bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let (base_shader_module, fragment_entry_point, fragment_shader_module) = create_shader(device);
    let fragment_shader_module = fragment_shader_module
        .as_ref()
        .unwrap_or(&base_shader_module);
    println!("Using fragment entry point '{fragment_entry_point}'");

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("main"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex_size = std::mem::size_of::<Vertex>();
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
            targets: &[Some(config.format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

pub fn create_bind_group(
    device: &Device,
    bind_group_layout: &BindGroupLayout,
    views: Vec<&TextureView>,
) -> BindGroup {
    let sampler = create_sampler(device);
    let mut texture_index_buffer_contents = vec![0u32; 128];
    texture_index_buffer_contents[0] = 0;
    texture_index_buffer_contents[1] = 1;
    let texture_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&texture_index_buffer_contents),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureViewArray(&views[..2]),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureViewArray(&views[2..]),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::SamplerArray(&[&sampler, &sampler]),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &texture_index_buffer,
                    offset: 0,
                    size: Some(NonZeroU64::new(4).unwrap()),
                }),
            },
        ],
        layout: bind_group_layout,
        label: Some("bind group"),
    })
}

pub fn create_sampler(device: &Device) -> Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor::default())
}
