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

use crate::{error::RuntimeError, settings::Settings};

use super::atlas::{Atlas, AtlasRegion};

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
    pub atlases: Option<[Atlas; 4]>,
    pub map_atlases: HashMap<usize, Atlas>,
    pub atlas_textures: RefCell<HashMap<String, Texture2D>>,
    pub image_atlas_coords: RefCell<HashMap<String, (f32, f32)>>,
}

pub struct Fonts {
    pub tahoma: Font,
}

pub struct UI {
    pub main: Texture2D,
}

impl Resources {
    pub async fn load(settings: &Settings) -> Self {
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

        let map_1_atlas = toml::from_str(
            &std::fs::read_to_string("./assets/map_1_atlas/output/map_1_atlas.toml").unwrap(),
        )
        .unwrap();
        let mut map_atlases = HashMap::new();
        map_atlases.insert(1, map_1_atlas);

        let resources = Resources {
            fonts,
            interface,
            images,
            animations,
            bodies,
            heads,
            weapons,
            body_templates,
            textures: RefCell::new(HashMap::new()),
            atlas_textures: RefCell::new(HashMap::new()),
            atlases: None,
            map_atlases,
            image_atlas_coords: RefCell::new(HashMap::new()),
        };

        // preload bodies, heads, weapons, etc (textures that constantly make GPU to swap)
        if settings.preload_textures {
            // for body in resources.bodies.values() {
            //     if let Some(image) = resources.get_body_image_file(body) {
            //         _ = resources.get_texture(image).await;
            //     }
            // }
            // for head in resources.heads.values() {
            //     if let Some(image) = resources.get_head_image_file(head) {
            //         _ = resources.get_texture(image as usize).await;
            //     }
            // }
            // for (_, weapons) in &resources.heads {
            //     let image = resources.get_head_image(head);
            //     _ = resources.get_texture(image.file_num as usize).await;
            // }
            // build_textures_atlas();
        }

        resources
    }

    pub fn get_head_image_file(&self, head: &Head) -> Option<u32> {
        Some(self.get_image(&head.0.to_string()).ok()?.file_num)
    }

    pub fn get_body_image_file(&self, body: &Body) -> Option<usize> {
        match body {
            Body::AnimatedWithTemplate { file_num, .. } => Some(*file_num),
            Body::Animated { walks, .. } => {
                let animation = walks.0;
                let animation = self.get_animation(&animation.to_string()).ok()?;
                let image = &animation.frames[0];
                Some(self.get_image(image).ok()?.file_num as usize)
            }
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

    pub async fn get_map_atlas_texture(&self, map: usize) -> Result<Texture2D, RuntimeError> {
        let id = format!("map_{map}_atlas.png");
        if !self.atlas_textures.borrow().contains_key(&id) {
            let Ok(texture) = load_texture(format!("./assets/map_{map}_atlas/output/{id}").as_str()).await else {
                    return Err(RuntimeError::TextureNotFound);
                };
            texture.set_filter(FilterMode::Nearest);
            self.atlas_textures.borrow_mut().insert(id.clone(), texture);
        }
        Ok(self.atlas_textures.borrow()[&id])
    }

    pub fn get_map_atlas_region(&self, map: usize, id: String) -> Option<&AtlasRegion> {
        let atlas = self.map_atlases.get(&map)?;

        atlas
            .regions
            .iter()
            .find(|atlas_region| atlas_region.name == id)
    }

    pub async fn get_atlas_texture(&self, layer: usize) -> Result<Texture2D, RuntimeError> {
        let id = format!("map_graphics_{layer}.png");
        if !self.atlas_textures.borrow().contains_key(&id) {
            let Ok(texture) = load_texture(format!("./assets/graphics/maps_{layer}/{id}").as_str()).await else {
                    return Err(RuntimeError::TextureNotFound);
                };
            texture.set_filter(FilterMode::Nearest);
            self.atlas_textures.borrow_mut().insert(id.clone(), texture);
        }
        Ok(self.atlas_textures.borrow()[&id])
    }

    pub fn get_atlas_region(&self, layer: usize, id: String) -> Option<&AtlasRegion> {
        let atlas = &self.atlases.as_ref().unwrap()[layer];

        atlas
            .regions
            .iter()
            .find(|atlas_region| atlas_region.name == id)
    }

    pub fn get_image_atlas_coords(&self, atlas_region: &AtlasRegion, image: &Image) -> (f32, f32) {
        *self
            .image_atlas_coords
            .borrow_mut()
            .entry(image.id.clone())
            .or_insert_with(|| {
                let texture_x = (atlas_region.x + image.x as u32) as f32;
                let texture_y =
                    (atlas_region.y + atlas_region.h - image.height as u32 - image.y as u32) as f32;
                (texture_x, texture_y)
            })
    }
}
