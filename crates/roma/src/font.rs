use std::io::Cursor;

use bmfont::{BMFont, OrdinateOrientation};

use crate::{renderer::texture, roma::get_state};

pub struct Fonts {
    pub font: BMFont,
}
pub const RESERVED_ID: usize = 40000;

impl Fonts {
    pub fn init() -> Self {
        let bmfont = BMFont::new(
            Cursor::new(&include_bytes!("fonts/font.fnt")[..]),
            OrdinateOrientation::BottomToTop,
        )
        .unwrap();
        Self { font: bmfont }
    }

    pub fn get_font(&self) -> &BMFont {
        &self.font
    }

    pub fn create_font_texture() -> (usize, texture::Texture) {
        let state = get_state();
        let image = image::load_from_memory(include_bytes!("fonts/shadowed-font.png")).unwrap();
        let font_texture =
            texture::Texture::from_image(&state.device, &state.queue, &image, Some("font"));

        (RESERVED_ID, font_texture)
    }
}
