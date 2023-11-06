use std::{io::Cursor, num::NonZeroUsize};

use bmfont::{BMFont, CharPosition, OrdinateOrientation};
use lru::LruCache;
use smol_str::SmolStr;

use crate::{renderer::texture, roma::get_state};

pub struct Fonts {
    pub font: BMFont,
    pub cache_glyphs: LruCache<SmolStr, (Vec<CharPosition>, u32)>,
}
pub const TEXT_ATLAS_TEXTURE_ID: u64 = 40000;

impl Fonts {
    pub fn init() -> Self {
        let bmfont = BMFont::new(
            Cursor::new(&include_bytes!("fonts/font.fnt")[..]),
            OrdinateOrientation::BottomToTop,
        )
        .unwrap();
        Self {
            font: bmfont,
            cache_glyphs: LruCache::new(NonZeroUsize::new(5000).unwrap()),
        }
    }

    pub fn parse(&mut self, text: SmolStr) -> Option<&(Vec<CharPosition>, u32)> {
        if self.cache_glyphs.get(&text).is_none() {
            let result = self.parse_text(&text);
            self.cache_glyphs.put(text.clone(), result);
        }

        self.cache_glyphs.get(&text)
    }

    pub fn parse_text(&mut self, text: &SmolStr) -> (Vec<CharPosition>, u32) {
        let chars = self.font.parse(text);

        chars.into_iter().fold(
            (Vec::with_capacity(text.len()), 0),
            |(mut char_positions, mut total_width), char| {
                let max_x = (char.screen_rect.x + char.screen_rect.width as i32) as u32;
                if total_width < max_x {
                    total_width = max_x;
                }
                char_positions.push(char);
                (char_positions, total_width)
            },
        )
    }

    pub fn create_font_texture() -> (u64, texture::Texture) {
        let state = get_state();
        let image = image::load_from_memory(include_bytes!("fonts/shadowed-font.png")).unwrap();
        let font_texture =
            texture::Texture::from_image(&state.device, &state.queue, &image, Some("font"));

        (TEXT_ATLAS_TEXTURE_ID, font_texture)
    }
}
