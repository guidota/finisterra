use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    num::{NonZeroU64, NonZeroUsize},
};

use lru::LruCache;
use wgpu::{
    util::{DeviceExt, StagingBelt},
    BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, Queue, Sampler, TextureView,
};

use crate::render::{
    sprite_batch::{SpriteData, Vertex},
    texture_array::{IndexedVertex, TextureArrayRenderPass},
};

use super::textures::Textures;

pub struct TextureArray {
    pub bind_group: Option<BindGroup>,
    pub textures: LruCache<String, u32>,

    pub default_texture: TextureView,
    pub texture_sampler: Sampler,
    pub texture_index_buffer: Buffer,
    vertex_buffers: HashMap<usize, (Vec<IndexedVertex>, Buffer)>,
}

impl TextureArray {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let white_texture_data = create_texture_data(Color::White);
        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d::default(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        };
        let white_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("white"),
            view_formats: &[],
            ..texture_descriptor
        });

        let white_texture_view = white_texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            white_texture.as_image_copy(),
            &white_texture_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: None,
            },
            wgpu::Extent3d::default(),
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let mut texture_index_buffer_contents = vec![0u32; 128];
        texture_index_buffer_contents[0] = 0;
        texture_index_buffer_contents[64] = 1;
        let texture_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&texture_index_buffer_contents),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let vertex_buffers = HashMap::new();
        let textures = LruCache::new(
            NonZeroUsize::new(TextureArrayRenderPass::MAX_TEXTURES as usize).unwrap(),
        );

        Self {
            bind_group: None,
            textures,
            default_texture: white_texture_view,
            texture_sampler: sampler,
            texture_index_buffer,
            vertex_buffers,
        }
    }

    pub fn update_vertex_buffer(
        &mut self,
        device: &Device,
        staging_belt: &mut StagingBelt,
        encoder: &mut CommandEncoder,
        entity_id: usize,
        indexed_vertices: Vec<IndexedVertex>,
    ) {
        match self.vertex_buffers.get_mut(&entity_id) {
            Some((vertices, buffer)) => {
                if vertices != &indexed_vertices {
                    println!("> update_vertex_buffer > diffent vertices for entity id {entity_id}");
                    let indexed_vertices_bytes = bytemuck::cast_slice(&indexed_vertices);
                    let mut vertex_buffer = staging_belt.write_buffer(
                        encoder,
                        buffer,
                        0,
                        NonZeroU64::new(indexed_vertices_bytes.len() as u64).unwrap(),
                        device,
                    );
                    vertex_buffer.copy_from_slice(indexed_vertices_bytes);
                    vertices.copy_from_slice(&indexed_vertices);
                }
            }
            None => {
                println!("> update_vertex_buffer > creating vertices");
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Sprite Batch Vertex Buffer"),
                    contents: bytemuck::cast_slice(&indexed_vertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
                self.vertex_buffers
                    .insert(entity_id, (indexed_vertices, vertex_buffer));
            }
        }
    }

    pub fn update_vertex_buffer_2(
        &mut self,
        device: &Device,
        staging_belt: &mut StagingBelt,
        encoder: &mut CommandEncoder,
        entity_id: usize,
        indexed_vertices: Vec<IndexedVertex>,
    ) {
        match self.vertex_buffers.get_mut(&entity_id) {
            Some((vertices, buffer)) => {
                if vertices != &indexed_vertices {
                    println!("> update_vertex_buffer > diffent vertices for entity id {entity_id}");
                    let vvertices: Vec<_> = indexed_vertices
                        .iter()
                        .map(|iv| Vertex {
                            position: [iv.pos[0], iv.pos[1], 0.],
                            tex_coords: iv.tex_coord,
                        })
                        .collect();
                    let indexed_vertices_bytes = bytemuck::cast_slice(&vvertices);
                    let mut vertex_buffer = staging_belt.write_buffer(
                        encoder,
                        buffer,
                        0,
                        NonZeroU64::new(indexed_vertices_bytes.len() as u64).unwrap(),
                        device,
                    );
                    vertex_buffer.copy_from_slice(indexed_vertices_bytes);
                    vertices.copy_from_slice(&indexed_vertices);
                }
            }
            None => {
                println!("> update_vertex_buffer > creating vertices");
                let vvertices: Vec<_> = indexed_vertices
                    .iter()
                    .map(|iv| Vertex {
                        position: [iv.pos[0], iv.pos[1], 0.],
                        tex_coords: iv.tex_coord,
                    })
                    .collect();

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Sprite Batch Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vvertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
                self.vertex_buffers
                    .insert(entity_id, (indexed_vertices, vertex_buffer));
            }
        }
    }

    pub fn get_vertex_buffer(&self, entity_id: usize) -> Option<&Buffer> {
        self.vertex_buffers
            .get(&entity_id)
            .map(|(_, buffer)| buffer)
    }

    pub fn push_texture(&mut self, texture_id: String, index: u32) {
        self.textures.put(texture_id, index);
    }

    pub fn update_bind_group(
        &mut self,
        texture_ids: HashSet<String>,
        textures: &Textures,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
    ) {
        let mut dirty = false;
        for texture_id in texture_ids {
            if self.textures.get(&texture_id).is_none() {
                let len = self.textures.len() as u32;
                if len < TextureArrayRenderPass::MAX_TEXTURES {
                    self.push_texture(texture_id, len);
                } else if let Some((_, index)) = self.textures.pop_lru() {
                    self.push_texture(texture_id, index);
                }
                dirty = true;
            }
        }
        if !dirty {
            return;
        }

        let mut texture_views =
            vec![&self.default_texture; TextureArrayRenderPass::MAX_TEXTURES as usize];
        let samplers = vec![&self.texture_sampler; TextureArrayRenderPass::MAX_TEXTURES as usize];

        for (id, index) in &self.textures {
            let (texture, _) = textures.get_texture(id).unwrap();
            texture_views[*index as usize] = texture.view.as_ref();
        }

        println!("> update_bind_group > creating bind group");
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&texture_views),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::SamplerArray(&samplers),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.texture_index_buffer,
                        offset: 0,
                        size: Some(NonZeroU64::new(4).unwrap()),
                    }),
                },
            ],
            layout: bind_group_layout,
            label: Some("bind group"),
        });

        self.bind_group = Some(bind_group);
    }

    pub fn get_index(&mut self, texture_id: &str) -> u32 {
        *self.textures.get(texture_id).unwrap_or(&0)
    }

    pub fn prepare_indexed_sprite_data(
        &mut self,
        sprites: &mut Vec<SpriteData>,
    ) -> (Vec<IndexedVertex>, Vec<String>) {
        if sprites.is_empty() {
            return (vec![], vec![]);
        }
        sprites.sort_unstable_by(|a, b| match a.z.partial_cmp(&b.z) {
            Some(Ordering::Equal) | None => a.texture_id.cmp(&b.texture_id),
            Some(other) => other,
        });
        sprites.reverse();

        let mut vertices = Vec::with_capacity(sprites.len() * 4);
        let mut texture_ids = HashSet::new();

        while let Some(sprite) = sprites.pop() {
            let texture_id = sprite.texture_id;
            let index = self.get_index(&texture_id);
            texture_ids.insert(texture_id);
            for vertex in sprite.vertices {
                vertices.push(IndexedVertex::from(vertex, index));
            }
        }

        (vertices, texture_ids.iter().cloned().collect())
    }
}

#[derive(Copy, Clone)]
enum Color {
    White,
}

fn create_texture_data(color: Color) -> [u8; 4] {
    match color {
        Color::White => [0, 0, 0, 0],
    }
}
