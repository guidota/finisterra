use rustc_hash::FxHashMap;

use crate::resources::texture::{self, Texture};

pub(crate) struct Textures {
    base_path: String,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    inner: FxHashMap<usize, InnerTexture>,
}

enum InnerTexture {
    Present {
        texture: texture::Texture,
        bind_group: wgpu::BindGroup,
    },
    NotPresent,
}

impl Textures {
    pub fn init(device: &wgpu::Device, base_path: &str) -> Self {
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
            base_path: base_path.to_string(),
            inner: FxHashMap::default(),
        }
    }

    pub(crate) fn load_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, id: usize) {
        if self.inner.contains_key(&id) {
            return;
        }
        let path = format!("{}/{}.png", self.base_path, id);
        let texture = match Texture::from_path(device, queue, &path) {
            Ok(texture) => {
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.bind_group_layout,
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

                InnerTexture::Present {
                    texture,
                    bind_group,
                }
            }
            _ => InnerTexture::NotPresent,
        };

        self.inner.insert(id, texture);
    }

    pub(crate) fn get_bind_group(&self, id: &usize) -> Option<&wgpu::BindGroup> {
        match self.inner.get(id) {
            Some(InnerTexture::Present { bind_group, .. }) => Some(bind_group),
            _ => None,
        }
    }

    pub(crate) fn get_texture(&self, id: &usize) -> Option<&texture::Texture> {
        match self.inner.get(id) {
            Some(InnerTexture::Present { texture, .. }) => Some(texture),
            _ => None,
        }
    }
}
