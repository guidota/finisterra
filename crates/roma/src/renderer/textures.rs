use engine::engine::TextureID;
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

pub struct Textures {
    textures: Vec<Texture>,
    files: IntMap<u32, String>,

    next_texture_id: usize,
}

impl Textures {
    pub fn initialize() -> Self {
        Self {
            textures: vec![],
            files: IntMap::default(),
            next_texture_id: 0,
        }
    }

    pub fn add_file(&mut self, path: &str) -> TextureID {
        let id = self.next_texture_id as TextureID;
        self.textures.push(Texture::Uninitialized);
        self.files.insert(id, path.to_string());
        self.next_texture_id += 1;
        id
    }

    pub fn set_file(&mut self, id: TextureID, path: &str) {
        let size = self.textures.len() as u32;
        if id > size {
            for _ in size..=id {
                self.textures.push(Texture::Uninitialized);
            }
        }

        self.files.insert(id, path.to_string());
        if id as usize > self.next_texture_id {
            self.next_texture_id = id as usize + 1;
        }
    }

    pub fn add_texture(&mut self, texture: texture::Texture) -> TextureID {
        let id = self.next_texture_id;
        self.textures.push(Texture::Present(texture));
        self.next_texture_id += 1;

        id as TextureID
    }

    pub fn load_texture(&mut self, device: &Device, queue: &Queue, id: TextureID) -> bool {
        match self.textures[id as usize] {
            Texture::Uninitialized => {
                if (id as usize) < self.next_texture_id {
                    self.next_texture_id = id as usize;
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

    pub fn get(&self, id: TextureID) -> Option<Option<&texture::Texture>> {
        match &self.textures[id as usize] {
            Texture::Uninitialized => None,
            Texture::NotFound => Some(None),
            Texture::Present(texture) => Some(Some(texture)),
        }
    }
}
