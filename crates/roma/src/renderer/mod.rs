use rustc_hash::FxHashMap;
use wgpu::util::DeviceExt;

use crate::{roma::get_state, Color, DrawImageParams};

pub(crate) mod texture;

type Texture = (wgpu::BindGroup, (usize, usize));

pub(crate) struct ImageRenderer {
    bind_group_layout: wgpu::BindGroupLayout,
    textures: FxHashMap<usize, Option<Texture>>,

    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    textures_folder: String,

    queue: FxHashMap<usize, Vec<DrawImageParams>>,
    sprites: Vec<Sprite>,
}

struct Instructions {
    batches: Vec<Batch>,
    sprites: usize,
}

#[derive(Default)]
struct Batch {
    texture_id: usize,
    size: u32,
}

impl ImageRenderer {
    // make this dynamic
    pub const MAX_SPRITES: usize = 50000;
    // const MAX_INDICES: usize = Self::MAX_SPRITES * 6;
    // const MAX_VERTICES: usize = Self::MAX_SPRITES * 4;
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
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Sprite>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32],
                }],
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
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let sprites_data = [Sprite::default(); Self::MAX_SPRITES];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Batch Storage Buffer"),
            contents: bytemuck::cast_slice(&sprites_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let textures = FxHashMap::default();

        Self {
            bind_group_layout,
            pipeline,
            queue: FxHashMap::default(),
            textures,
            textures_folder: textures_folder.to_string(),
            sprites: vec![Sprite::default(); Self::MAX_SPRITES],
            vertex_buffer,
        }
    }

    pub(crate) fn add_texture(&mut self, id: usize, texture: &texture::Texture) {
        self.textures
            .insert(id, Some(texture.to_bind_group(&self.bind_group_layout)));
    }

    fn load_texture(&mut self, id: &usize) {
        if self.textures.contains_key(id) {
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

        self.textures.insert(*id, texture);
    }

    pub fn queue(&mut self, params: DrawImageParams) {
        let id = params.texture_id;
        self.queue
            .entry(id)
            .or_insert_with(|| Vec::with_capacity(Self::MAX_SPRITES))
            .push(params);
    }

    pub fn queue_multiple(&mut self, texture_id: usize, params: &mut Vec<DrawImageParams>) {
        self.queue
            .entry(texture_id)
            .or_insert_with(|| Vec::with_capacity(Self::MAX_SPRITES))
            .append(params);
    }

    fn process_queue(&mut self) -> Instructions {
        let mut batches = vec![];

        let texture_ids: Vec<_> = self.queue.keys().copied().collect();
        texture_ids.iter().for_each(|id| self.load_texture(id));

        let mut sprite_index = 0;
        for texture_id in texture_ids {
            let Some(draws) = self.queue.get_mut(&texture_id) else {
                continue;
            };
            let Some(Some((_, dimensions))) = self.textures.get(&texture_id) else {
                continue;
            };
            let batch = Batch {
                texture_id,
                size: draws.len() as u32,
            };

            batches.push(batch);
            for draw in draws.drain(..) {
                draw.update_sprite(&mut self.sprites[sprite_index], dimensions);
                sprite_index += 1;
            }
        }

        Instructions {
            batches,
            sprites: sprite_index,
        }
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
        let sprite_data = bytemuck::cast_slice(&self.sprites[..instructions.sprites]);
        get_state()
            .queue
            .write_buffer(&self.vertex_buffer, 0, sprite_data);

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(1, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        let mut offset = 0;
        for Batch { texture_id, size } in instructions.batches {
            if let Some(Some((bind_group, _))) = self.textures.get(&texture_id) {
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.draw(0..4, offset..(offset + size));
            }
            offset += size;
        }
    }
}

impl DrawImageParams {
    fn update_sprite(self, sprite: &mut Sprite, dimensions: &(usize, usize)) {
        let (texture_width, texture_height) = (dimensions.0 as f32, dimensions.1 as f32);

        let source = self
            .source
            .unwrap_or([0., 0., texture_width, texture_height]);
        let color = self.color;
        let flip_y = self.flip_y;
        let [x, y, z] = self.position;
        let [sx, sy, sw, sh] = source;
        let p = [
            [x, y],
            [x + sw, y + sh],
            // [x + sw, y + sh, z],
            // [x, y + sh, z],
        ];

        let mut tex_coords = [
            [sx / texture_width, (sy + sh) / texture_height],
            [(sx + sw) / texture_width, sy / texture_height],
            // [(sx + sw) / texture_width, (sy + sh) / texture_height],
            // [sx / texture_width, sy / texture_height],
        ];

        if flip_y {
            tex_coords.swap(0, 3);
            tex_coords.swap(1, 2);
        }

        sprite.top_left = p[0];
        sprite.bottom_right = p[1];

        sprite.tex_top_left = tex_coords[0];
        sprite.tex_bottom_right = tex_coords[1];

        sprite.color = color;
        sprite.z = z;
    }
}

#[derive(Default, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
struct Sprite {
    top_left: [f32; 2],
    bottom_right: [f32; 2],

    tex_top_left: [f32; 2],
    tex_bottom_right: [f32; 2],

    color: Color,
    z: f32,
}
