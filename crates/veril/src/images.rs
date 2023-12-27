use nohash_hasher::IntMap;
use wgpu::{Device, Queue};

use crate::texture;

pub struct Images {
    textures: IntMap<u64, Option<texture::Texture>>,
    files: IntMap<u64, String>,
}

impl Images {
    pub fn initialize() -> Self {
        Self {
            textures: IntMap::default(),
            files: IntMap::default(),
        }
    }

    pub fn add_file(&mut self, path: &str) -> u64 {
        let id = self.files.len() as u64;
        self.files.insert(id, path.to_string());
        id
    }

    pub fn set_file(&mut self, id: u64, path: &str) {
        self.files.insert(id, path.to_string());
    }

    pub fn add_texture(&mut self, texture: texture::Texture) -> u64 {
        let id = *self.textures.keys().max().unwrap_or(&0) + 1;
        self.textures.insert(id, Some(texture));
        id
    }

    pub fn load_texture(&mut self, device: &Device, queue: &Queue, id: u64) -> bool {
        if let Some(texture) = self.textures.get(&id) {
            return texture.is_some();
        }

        let Some(path) = self.files.get(&id) else {
            self.textures.insert(id, None);
            log::error!("Path not found for texture {id}");
            return false;
        };

        let Ok(texture) = texture::Texture::from_path(device, queue, path) else {
            log::error!("Texture not found on {path}");
            self.textures.insert(id, None);
            return false;
        };

        self.textures.insert(id, Some(texture));

        true
    }

    pub fn get(&self, id: u64) -> Option<&Option<texture::Texture>> {
        self.textures.get(&id)
    }
}
