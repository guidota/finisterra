use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Atlas {
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
