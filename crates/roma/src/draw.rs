use crate::resources::texture::Texture;

#[derive(Default, Debug, Clone)]
pub struct DrawParams {
    pub texture_id: usize,
    pub x: usize,
    pub y: usize,
    pub z: f32,
    pub source: Option<Rect>,
    pub flip_y: bool,
}

#[derive(Default, Debug)]
pub struct DrawStrictParams {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sx: f32,
    pub sy: f32,
    pub sw: f32,
    pub sh: f32,
    pub texture_width: f32,
    pub texture_height: f32,
    pub flip_y: bool,
}

impl DrawParams {
    pub fn to_strict(self, texture: &Texture) -> DrawStrictParams {
        let source = self.source.unwrap_or(Rect {
            x: 0,
            y: 0,
            w: texture.width as usize,
            h: texture.height as usize,
        });
        DrawStrictParams {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z,
            sx: source.x as f32,
            sy: source.y as f32,
            sw: source.w as f32,
            sh: source.h as f32,
            texture_width: texture.width as f32,
            texture_height: texture.height as f32,
            flip_y: self.flip_y,
        }
    }
}

/// A 2D rectangle, defined by its top-left corner, width and height.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }
}
