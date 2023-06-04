use wgpu::util::DeviceExt;

use super::vertex::Vertex;

pub(crate) struct RenderPass {
    pub(crate) depth_texture_view: wgpu::TextureView,
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) vertex_buffer: wgpu::Buffer,
}

impl RenderPass {
    pub const MAX_SPRITES: usize = 8196;
    const MAX_INDICES: usize = Self::MAX_SPRITES * 6;
    const MAX_VERTICES: usize = Self::MAX_SPRITES * 4;

    pub fn init(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let pipeline = device.create_sprite_pipeline(
            config,
            &[texture_bind_group_layout, camera_bind_group_layout],
        );

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

        let depth_texture_view = create_depth_texture(device, config);

        Self {
            pipeline,
            index_buffer,
            vertex_buffer,
            depth_texture_view,
        }
    }

    pub(crate) fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.depth_texture_view = create_depth_texture(device, config);
    }
}

trait DeviceSpriteBatchPipelineExt {
    fn create_sprite_pipeline(
        &self,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline;
}

impl DeviceSpriteBatchPipelineExt for wgpu::Device {
    fn create_sprite_pipeline(
        &self,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
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
                targets: &[Some(create_color_targets(config))],
            }),
            primitive: create_primitive_state(),
            depth_stencil: Some(create_depth_state()),
            multisample: create_multisample_state(),
            multiview: None,
        })
    }
}

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24PlusStencil8;

pub fn create_depth_texture(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[DEPTH_FORMAT],
    };
    let texture = device.create_texture(&desc);

    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn create_depth_state() -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::GreaterEqual,
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }
}

pub fn create_depth_attachment(view: &wgpu::TextureView) -> wgpu::RenderPassDepthStencilAttachment {
    wgpu::RenderPassDepthStencilAttachment {
        view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(0.0),
            store: true,
        }),

        stencil_ops: None,
    }
}

pub fn create_color_attachment(view: &wgpu::TextureView) -> wgpu::RenderPassColorAttachment {
    wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: true,
        },
    }
}

fn create_color_targets(config: &wgpu::SurfaceConfiguration) -> wgpu::ColorTargetState {
    wgpu::ColorTargetState {
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
    }
}

fn create_primitive_state() -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: true,
        conservative: false,
    }
}

fn create_multisample_state() -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
}
