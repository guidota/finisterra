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
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
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
    pub width: usize,
    pub height: usize,
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
            width: value.width as usize,
            height: value.height as usize,
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
                    x: region.x as usize,
                    y: region.y as usize,
                    width: region.w as usize,
                    height: region.h as usize,
                },
            });
            width = region.atlas_width as usize;
            height = region.atlas_height as usize;
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
        images_by_file_num: &FxHashMap<usize, Vec<usize>>,
        atlas_file_num: usize,
    ) {
        // for each atlas region, find image and calculate coordinates in the texture
        for atlas_item in &self.images {
            let image_id = atlas_item.image_id;

            let Some(image_ids) = images_by_file_num.get(&image_id) else {
                continue;
            };

            for image_id in image_ids {
                let Some(image) = images.get_mut(image_id) else {
                    continue;
                };
                image.x += atlas_item.rect.x;
                image.y += atlas_item.rect.y;
                // ensure image is not bigger than atlas item
                image.width = std::cmp::min(image.width, atlas_item.rect.width);
                image.height = std::cmp::min(image.height, atlas_item.rect.height);

                image.file_num = atlas_file_num;
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
