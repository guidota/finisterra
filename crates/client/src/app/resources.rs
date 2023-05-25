use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

use ao::ao_20::{
    graphics::{Animation, Image},
    init::{
        parse::{
            body::parse_bodies,
            head::parse_heads,
            template::{parse_templates, Template},
            weapon::parse_weapons,
        },
        Body, Head, Weapon,
    },
};
use macroquad::prelude::*;

use crate::error::RuntimeError;

pub struct Resources {
    pub fonts: Fonts,
    pub images: BTreeMap<String, Image>,
    pub animations: BTreeMap<String, Animation>,
    pub bodies: BTreeMap<usize, Body>,
    pub heads: BTreeMap<usize, Head>,
    pub weapons: BTreeMap<usize, Weapon>,
    pub body_templates: BTreeMap<usize, Template>,
    // pub shields: BTreeMap<String, Shield>,
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

        let bodies = parse_bodies("./assets/init/cuerpos.dat").expect("can parse bodies");
        let heads = parse_heads("./assets/init/cabezas.ini");
        let weapons = parse_weapons("./assets/init/armas.dat").expect("can parse weapons");
        let body_templates = parse_templates("./assets/init/moldes.ini");

        Resources {
            fonts,
            interface,
            images,
            animations,
            bodies,
            heads,
            weapons,
            body_templates,
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
