use nohash_hasher::IntMap;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::texture;

pub type Texture = (wgpu::BindGroup, (usize, usize));

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

    pub fn add_texture(&mut self, path: &str) -> u64 {
        let id = self.files.len() as u64;
        self.files.insert(id, path.to_string());
        id
    }

    pub fn set_texture(&mut self, id: u64, path: &str) {
        self.files.insert(id, path.to_string());
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

        let texture = match texture::Texture::from_path(device, queue, path) {
            Ok(texture) => Some(texture.to_bind_group(device, bind_group_layout)),
            _ => {
                log::error!("Texture not found on {path}");
                None
            }
        };

        self.textures.insert(id, texture);
    }
}
