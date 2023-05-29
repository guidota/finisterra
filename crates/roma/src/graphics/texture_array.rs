use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    num::NonZeroU64,
};

use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device, Queue, Sampler, TextureView,
};

use crate::render::{
    sprite_batch::SpriteData,
    texture_array::{IndexedVertex, TextureArrayRenderPass},
};

use super::textures::Textures;

pub struct TextureArray {
    pub bind_group: Option<BindGroup>,
    pub texture_ids: Vec<String>,
    pub texture_index: HashMap<String, u32>,
    pub default_texture: TextureView,
    pub texture_sampler: Sampler,
    pub texture_index_buffer: Buffer,
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

        Self {
            bind_group: None,
            texture_ids: vec![],
            texture_index: HashMap::new(),
            default_texture: white_texture_view,
            texture_sampler: sampler,
            texture_index_buffer,
        }
    }

    fn should_update_bind_group(&self, texture_ids: &Vec<String>) -> bool {
        !self.texture_ids.eq(texture_ids)
    }

    pub fn update_bind_group(
        &mut self,
        texture_ids: Vec<String>,
        textures: &Textures,
        device: &Device,
        bind_group_layout: &BindGroupLayout,
    ) {
        if self.bind_group.is_some() && !self.should_update_bind_group(&texture_ids) {
            return;
        }
        self.texture_index.clear();
        self.texture_ids = texture_ids;
        let mut texture_views = vec![];
        let mut samplers = vec![];
        for i in 0..TextureArrayRenderPass::MAX_TEXTURES {
            let i = i as usize;
            if self.texture_ids.len() > i {
                let id = &self.texture_ids[i];
                if let Some((texture, _)) = textures.get_texture(id) {
                    texture_views.push(texture.view.as_ref());
                    self.texture_index.insert(id.to_string(), i as u32);
                } else {
                    texture_views.push(&self.default_texture);
                }
            } else {
                texture_views.push(&self.default_texture);
            }
            samplers.push(&self.texture_sampler);
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

    pub fn get_index(&self, texture_id: &str) -> u32 {
        *self.texture_index.get(texture_id).unwrap_or(&0)
    }

    pub fn prepare_indexed_sprite_data(
        &self,
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
        Color::White => [255, 255, 255, 0],
    }
}
