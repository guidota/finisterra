use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::image::Image;

/// Struct used by texture-packer (https://github.com/ii887522/texture-packer/)
/// Copied to deserialize the atlas file
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TexturePackerAtlas {
    pub regions: Vec<AtlasRegion>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AtlasRegion {
    pub name: String,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub is_rotated: bool,
    pub is_opaque: bool,
}

/// Struct used by yatp (https://github.com/pepetrov0/yatp/)
/// Copied to deserialize the atlas file
/// Dictionary containing data about the packed textures and texture atlas
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Dictionary {
    width: u32,
    height: u32,
    items: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Entry {
    pub name: String,
    pub path: String,
    pub rect: Rect,
}

/// Atlas type
pub enum AtlasType {
    TexturePacker,
    Yatp,
    Finisterra,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Atlas {
    pub images: Vec<AtlasItem>,
    pub width: u32,
    pub height: u32,
}

/// Describes where to find an image in the atlas
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AtlasItem {
    pub image_id: usize,

    #[serde(flatten)]
    pub rect: Rect,
}

impl From<Dictionary> for Atlas {
    fn from(value: Dictionary) -> Self {
        let mut images = Vec::new();

        for item in value.items {
            images.push(AtlasItem {
                image_id: item.name.parse::<usize>().unwrap(),
                rect: item.rect,
            });
        }

        Self {
            images,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<TexturePackerAtlas> for Atlas {
    fn from(value: TexturePackerAtlas) -> Self {
        let mut images = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for region in value.regions {
            images.push(AtlasItem {
                image_id: region.name.parse::<usize>().unwrap(),
                rect: Rect {
                    x: region.x,
                    y: region.y,
                    width: region.w,
                    height: region.h,
                },
            });
            width = region.atlas_width;
            height = region.atlas_height;
        }

        Self {
            images,
            width,
            height,
        }
    }
}

impl Atlas {
    pub fn update_images(
        &self,
        images: &mut FxHashMap<usize, Image>,
        images_by_file_num: &FxHashMap<u32, Vec<usize>>,
        atlas_file_num: usize,
    ) {
        // for each atlas region, find image and calculate coordinates in the texture
        for atlas_item in &self.images {
            let image_id = atlas_item.image_id;

            let Some(image_ids) = images_by_file_num.get(&(image_id as u32)) else {
                continue;
            };

            for image_id in image_ids {
                let Some(image) = images.get_mut(image_id) else {
                    continue;
                };

                image.x += atlas_item.rect.x as u16;
                image.y += atlas_item.rect.y as u16;
                image.file_num = atlas_file_num as u32;
            }
        }
    }
}

pub struct AtlasResource<'a> {
    pub metadata_path: &'a str,
    /// Atlas image should be {image_id}.png
    pub image_id: usize,
    pub atlas_type: AtlasType,
}
