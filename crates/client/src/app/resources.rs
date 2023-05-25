use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

use ao::ao_20::graphics::{Animation, Image};
use macroquad::prelude::*;

use crate::error::RuntimeError;

pub struct Resources {
    pub fonts: Fonts,
    pub images: BTreeMap<String, Image>,
    pub animations: BTreeMap<String, Animation>,
    pub textures: RefCell<HashMap<usize, Texture2D>>,
    pub interface: UI,
}

pub struct Fonts {
    pub tahoma: Font,
}

pub struct UI {
    pub main: Texture2D,
}

impl Resources {
    pub async fn load() -> Self {
        let (images, animations) =
            ao::ao_20::graphics::parse::parse_graphics("./assets/init/graficos.ind")
                .expect("can parse graphics");

        let fonts = Fonts {
            tahoma: load_ttf_font("./assets/fonts/tahoma_bold.ttf")
                .await
                .expect("Can load font"),
        };
        let interface = UI {
            main: load_texture("./assets/interface/main_inv.png")
                .await
                .expect("can load UI"),
        };
        Resources {
            fonts,
            interface,
            images,
            animations,
            textures: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_image(&self, id: &str) -> Result<&Image, RuntimeError> {
        self.images.get(id).ok_or(RuntimeError::ImageNotFound)
    }

    pub fn get_animation(&self, id: &str) -> Result<&Animation, RuntimeError> {
        self.animations
            .get(id)
            .ok_or(RuntimeError::AnimationNotFound)
    }

    pub async fn get_texture(&self, id: usize) -> Result<Texture2D, RuntimeError> {
        if !self.textures.borrow().contains_key(&id) {
            let Ok(texture) = load_texture(format!("./assets/graphics/{}.png", id).as_str()).await else {
                    return Err(RuntimeError::TextureNotFound);
                };
            texture.set_filter(FilterMode::Nearest);
            self.textures.borrow_mut().insert(id, texture);
        }
        Ok(self.textures.borrow()[&id])
    }
}
