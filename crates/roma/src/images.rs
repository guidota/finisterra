use nohash_hasher::IntMap;
use wgpu::{Device, Queue};

use crate::texture;

#[derive(Default)]
enum Texture {
    #[default]
    Uninitialized,
    NotFound,
    Present(texture::Texture),
}

pub struct Images {
    textures: Vec<Texture>,
    files: IntMap<u64, String>,

    max_texture_id: usize,
}

impl Images {
    pub const MAX_IMAGES: u64 = 100000;

    pub fn initialize() -> Self {
        let mut textures = Vec::with_capacity(Self::MAX_IMAGES as usize);
        for _ in 0..Self::MAX_IMAGES {
            textures.push(Texture::Uninitialized);
        }

        Self {
            textures,
            files: IntMap::default(),
            max_texture_id: 0,
        }
    }

    pub fn add_file(&mut self, path: &str) -> u64 {
        let id = self.files.len() as u64;
        if id as usize > self.max_texture_id {
            self.max_texture_id = id as usize;
        }
        self.files.insert(id, path.to_string());
        id
    }

    pub fn set_file(&mut self, id: u64, path: &str) {
        self.files.insert(id, path.to_string());
        if id as usize > self.max_texture_id {
            self.max_texture_id = id as usize;
        }
    }

    pub fn add_texture(&mut self, texture: texture::Texture) -> u64 {
        let mut id: u64 = self.max_texture_id as u64 + 1;
        for i in self.max_texture_id + 1..Self::MAX_IMAGES as usize {
            if matches!(self.textures[i], Texture::Uninitialized) {
                id = i as u64;
                self.textures[i] = Texture::Present(texture);
                break;
            }
        }
        self.max_texture_id = id as usize;
        id
    }

    pub fn load_texture(&mut self, device: &Device, queue: &Queue, id: u64) -> bool {
        match self.textures[id as usize] {
            Texture::Uninitialized => {
                if (id as usize) < self.max_texture_id {
                    self.max_texture_id = id as usize;
                }
                let Some(path) = self.files.get(&id) else {
                    self.textures[id as usize] = Texture::NotFound;
                    log::error!("Path not found for texture {id}");
                    return false;
                };

                let Ok(texture) = texture::Texture::from_path(device, queue, path) else {
                    log::error!("Texture not found on {path}");
                    self.textures[id as usize] = Texture::NotFound;
                    return false;
                };

                self.textures[id as usize] = Texture::Present(texture);

                true
            }
            Texture::NotFound => false,
            Texture::Present(_) => true,
        }
    }

    pub fn get(&self, id: u64) -> Option<Option<&texture::Texture>> {
        match &self.textures[id as usize] {
            Texture::Uninitialized => None,
            Texture::NotFound => Some(None),
            Texture::Present(texture) => Some(Some(texture)),
        }
    }
}
