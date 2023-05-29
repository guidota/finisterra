use std::collections::HashMap;

use wgpu::{BindGroup, BindGroupLayout, Color, Device};

use crate::{render::sprite_batch::Vertex, resources::texture::Texture};

use super::{rect::Rect, vec2::vec2, Graphics, SpriteData};

pub struct Textures {
    pub bind_group_layout: BindGroupLayout,
    pub collection: HashMap<String, (Texture, BindGroup)>,
}

impl Textures {
    pub fn new(device: &Device) -> Textures {
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

        Self {
            bind_group_layout,
            collection: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, id: String, texture: (Texture, BindGroup)) {
        self.collection.insert(id, texture);
    }

    pub fn has_texture(&self, id: &str) -> bool {
        self.collection.contains_key(id)
    }

    pub fn get_texture(&self, id: &str) -> Option<&(Texture, BindGroup)> {
        self.collection.get(id)
    }
}

#[derive(Default, Debug, Clone)]
pub struct DrawTextureParams {
    pub source: Option<Rect>,
    pub flip_y: bool,
}

impl Graphics {
    pub fn load_texture<ID: Into<String>>(&mut self, id: ID, path: &str) {
        let id = id.into();
        if self.textures.has_texture(&id) {
            return;
        }
        let Ok(texture) = Texture::from_path(&self.device, &self.queue, path) else {
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
        self.textures.add_texture(id, (texture, bind_group));
        println!(
            "> load_textures > loaded textures {}",
            self.textures.collection.len()
        );
    }

    pub fn draw_texture<ID: Into<String>>(
        &mut self,
        texture_id: ID,
        x: f32,
        y: f32,
        z: usize,
        _color: Color,
        params: Option<DrawTextureParams>,
    ) {
        let id = texture_id.into();
        let Some((texture, _)) = self.textures.get_texture(&id) else {
            return;
        };

        let params = params.unwrap_or_default();
        let Rect {
            x: sx,
            y: sy,
            w: sw,
            h: sh,
        } = params.source.unwrap_or(Rect {
            x: 0.,
            y: 0.,
            w: texture.width as f32,
            h: texture.height as f32,
        });

        let (width, height) = (texture.width as f32, texture.height as f32);

        let (w, h) = (sw, sh);

        let p = [
            vec2(x, y),
            vec2(x + w, y),
            vec2(x + w, y + h),
            vec2(x, y + h),
        ];

        let mut tex_coords = [
            vec2(sx / width, sy / height),
            vec2((sx + sw) / width, sy / height),
            vec2((sx + sw) / width, (sy + sh) / height),
            vec2(sx / width, (sy + sh) / height),
        ];

        if params.flip_y {
            tex_coords.swap(0, 3);
            tex_coords.swap(1, 2);
        }

        let vertices = vec![
            Vertex::new(p[0], tex_coords[0]),
            Vertex::new(p[1], tex_coords[1]),
            Vertex::new(p[2], tex_coords[2]),
            Vertex::new(p[3], tex_coords[3]),
        ];

        let sprite_data = SpriteData {
            z,
            texture_id: id,
            vertices,
        };
        self.frame_draws.push(sprite_data);
    }
}
