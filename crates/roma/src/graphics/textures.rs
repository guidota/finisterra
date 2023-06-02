use std::collections::HashMap;

use object_pool::Pool;
use wgpu::{BindGroup, BindGroupLayout, Device};

use crate::{render::sprite_batch::Vertex, resources::texture::Texture};

use super::{rect::Rect, Graphics, SpriteData};

pub struct Textures {
    pub bind_group_layout: BindGroupLayout,
    pub collection: HashMap<usize, (Texture, BindGroup)>,
    // path where graphics are stored
    pub base_path: String,

    pub sprite_data_pool: Pool<SpriteData>,
}

impl Textures {
    pub fn new(device: &Device, base_path: String) -> Textures {
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

        let sprite_data_pool = Pool::new(1000, SpriteData::default);
        Self {
            bind_group_layout,
            collection: HashMap::new(),
            sprite_data_pool,
            base_path,
        }
    }

    pub fn add_texture(&mut self, id: usize, texture: (Texture, BindGroup)) {
        self.collection.insert(id, texture);
    }

    pub fn has_texture(&self, id: &usize) -> bool {
        self.collection.contains_key(id)
    }

    pub fn get_texture(&self, id: &usize) -> Option<&(Texture, BindGroup)> {
        self.collection.get(id)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DrawParams {
    pub texture_id: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub source: Option<Rect>,
    pub flip_y: bool,
}

#[derive(Default, Debug)]
pub struct DrawStrictParams {
    pub x: f32,
    pub y: f32,
    pub sx: f32,
    pub sy: f32,
    pub sw: f32,
    pub sh: f32,
    pub texture_width: f32,
    pub texture_height: f32,
    pub flip_y: bool,
}

impl DrawParams {
    pub fn to_strict(self, texture: &Texture) -> DrawStrictParams {
        let source = self.source.unwrap_or(Rect {
            x: 0,
            y: 0,
            w: texture.width as usize,
            h: texture.height as usize,
        });
        DrawStrictParams {
            x: self.x as f32,
            y: self.y as f32,
            sx: source.x as f32,
            sy: source.y as f32,
            sw: source.w as f32,
            sh: source.h as f32,
            texture_width: texture.width as f32,
            texture_height: texture.height as f32,
            flip_y: self.flip_y,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct DrawTextureParams {
    pub source: Option<Rect>,
    pub flip_y: bool,
}

impl Graphics {
    pub fn load_texture(&mut self, id: &usize) {
        if self.textures.has_texture(id) {
            return;
        }
        let path = format!("{}/{}.png", self.textures.base_path, id);
        let Ok(texture) = Texture::from_path(&self.device, &self.queue, &path) else {
            println!("> texture not loaded > {id}");
            return;
        };

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.textures.bind_group_layout,
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
            label: Some("diffuse_bind_group"),
        });
        println!(
            "> load_textures > loaded texture {} -- sum {}",
            id,
            self.textures.collection.len()
        );
        self.textures.add_texture(*id, (texture, bind_group));
    }

    pub fn draw_texture(&mut self, entity_id: usize, params: DrawParams) {
        let Some((texture, _)) = self.textures.get_texture(&params.texture_id) else {
            println!("> draw_texture > texture not found > {}", params.texture_id);
            return;
        };

        let sprite_data = SpriteData {
            entity_id,
            z: params.z,
            texture_id: params.texture_id,
        };

        let params = params.to_strict(texture);
        let vertices = self.create_vertices(params);

        self.push_draw(sprite_data, vertices);
    }

    pub fn create_vertices(&mut self, params: DrawStrictParams) -> Vec<Vertex> {
        let DrawStrictParams {
            texture_width,
            texture_height,
            flip_y,
            x,
            y,
            sx,
            sy,
            sw,
            sh,
        } = params;

        let p = [
            [x, y, 0.],
            [x + sw, y, 0.],
            [x + sw, y + sh, 0.],
            [x, y + sh, 0.],
        ];

        let mut tex_coords = [
            [sx / texture_width, sy / texture_height],
            [(sx + sw) / texture_width, sy / texture_height],
            [(sx + sw) / texture_width, (sy + sh) / texture_height],
            [sx / texture_width, (sy + sh) / texture_height],
        ];

        if flip_y {
            tex_coords.swap(0, 3);
            tex_coords.swap(1, 2);
        }

        let mut vertices = vec![];
        for i in 0..4 {
            vertices.push(Vertex {
                position: p[i],
                tex_coords: tex_coords[i],
            });
        }
        vertices
    }
}
