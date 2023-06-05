use rustc_hash::FxHashMap;

use crate::resources::texture::{self, Texture};

pub(crate) struct Textures {
    base_path: String,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    textures: FxHashMap<TextureID, InnerTexture>,
}

enum InnerTexture {
    Present {
        texture: texture::Texture,
        bind_group: wgpu::BindGroup,
    },
    NotPresent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TextureID {
    Image(usize),
    Glyph(usize),
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
        let textures = FxHashMap::default();

        Self {
            bind_group_layout,
            base_path: base_path.to_string(),
            textures,
        }
    }

    pub(crate) fn recreate_texture(
        &mut self,
        device: &wgpu::Device,
        id: TextureID,
        dimensions: (u32, u32),
    ) {
        let texture = Texture::from_dimensions_text(device, dimensions);
        let bind_group = self.create_bind_group(device, &texture);

        let texture = InnerTexture::Present {
            texture,
            bind_group,
        };

        self.textures.insert(id, texture);
    }

    pub(crate) fn load_image(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_id: usize,
    ) {
        let id = TextureID::Image(image_id);
        if self.textures.contains_key(&id) {
            return;
        }
        let path = format!("{}/{}.png", self.base_path, image_id);
        let texture = match Texture::from_path(device, queue, &path) {
            Ok(texture) => {
                let bind_group = self.create_bind_group(device, &texture);

                InnerTexture::Present {
                    texture,
                    bind_group,
                }
            }
            _ => InnerTexture::NotPresent,
        };

        self.textures.insert(id, texture);
    }

    fn create_bind_group(&self, device: &wgpu::Device, texture: &Texture) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
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
        })
    }

    pub(crate) fn get_bind_group(&self, id: &TextureID) -> Option<&wgpu::BindGroup> {
        match self.textures.get(id) {
            Some(InnerTexture::Present { bind_group, .. }) => Some(bind_group),
            _ => None,
        }
    }

    pub(crate) fn get_texture(&self, id: &TextureID) -> Option<&texture::Texture> {
        match self.textures.get(id) {
            Some(InnerTexture::Present { texture, .. }) => Some(texture),
            _ => None,
        }
    }
}
