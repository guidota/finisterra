use std::io::Cursor;

use bmfont::{BMFont, CharPosition, OrdinateOrientation};
use rustc_hash::FxHashMap;
use smol_str::SmolStr;

use crate::{renderer::texture, roma::get_state};

pub struct Fonts {
    pub font: BMFont,
    pub cache_glyphs: FxHashMap<SmolStr, (Vec<CharPosition>, u32)>,
}
pub const RESERVED_ID: usize = 40000;

impl Fonts {
    pub fn init() -> Self {
        let bmfont = BMFont::new(
            Cursor::new(&include_bytes!("fonts/font.fnt")[..]),
            OrdinateOrientation::BottomToTop,
        )
        .unwrap();
        Self {
            font: bmfont,
            cache_glyphs: FxHashMap::default(),
        }
    }

    pub fn parse(&mut self, text: SmolStr) -> Option<&(Vec<CharPosition>, u32)> {
        if !self.cache_glyphs.contains_key(&text) {
            let chars = self.font.parse(&text);
            let result = chars.into_iter().fold(
                (Vec::with_capacity(text.len()), 0),
                |(mut char_positions, mut total_width), char| {
                    total_width += char.screen_rect.width;
                    char_positions.push(char);
                    (char_positions, total_width)
                },
            );
            self.cache_glyphs.insert(text.clone(), result);
        }

        self.cache_glyphs.get(&text)
    }

    pub fn create_font_texture() -> (usize, texture::Texture) {
        let state = get_state();
        let image = image::load_from_memory(include_bytes!("fonts/shadowed-font.png")).unwrap();
        let font_texture =
            texture::Texture::from_image(&state.device, &state.queue, &image, Some("font"));

        (RESERVED_ID, font_texture)
    }
}
