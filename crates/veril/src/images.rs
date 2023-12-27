use nohash_hasher::IntMap;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::texture;

pub struct Texture {
    pub bind_group: wgpu::BindGroup,
    pub texture: texture::Texture,
}

pub struct Images {
    textures: IntMap<u64, Option<Texture>>,
    files: IntMap<u64, String>,
}

impl Images {
    pub fn initialize() -> Self {
        Self {
            textures: IntMap::default(),
            files: IntMap::default(),
        }
    }

    pub fn textures(&self) -> &IntMap<u64, Option<Texture>> {
        &self.textures
    }

    pub fn add_file(&mut self, path: &str) -> u64 {
        let id = self.files.len() as u64;
        self.files.insert(id, path.to_string());
        id
    }

    pub fn set_file(&mut self, id: u64, path: &str) {
        self.files.insert(id, path.to_string());
    }

    pub fn add_texture(&mut self, texture: Texture) -> u64 {
        let id = *self.textures.keys().max().unwrap_or(&0) + 100000;
        self.textures.insert(id, Some(texture));
        id
    }

    pub fn load_texture(
        &mut self,
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
        id: u64,
    ) {
        if self.textures.contains_key(&id) {
            return;
        }
        let Some(path) = self.files.get(&id) else {
            self.textures.insert(id, None);
            log::error!("Path not found for texture {id}");
            return;
        };

        let Ok(texture) = texture::Texture::from_path(device, queue, path) else {
            log::error!("Texture not found on {path}");
            self.textures.insert(id, None);
            return;
        };
        let bind_group = texture.create_bind_group(device, bind_group_layout);

        self.textures.insert(
            id,
            Some(Texture {
                texture,
                bind_group,
            }),
        );
    }
}
