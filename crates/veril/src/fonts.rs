use std::{collections::HashMap, io::Cursor};

use bmfont::{BMFont, CharPosition, OrdinateOrientation};
use nohash_hasher::IntMap;

use crate::files::read_file;

pub type ParsedText = (Vec<CharPosition>, u32);
pub struct Font {
    texture_id: u64,
    bmfont: bmfont::BMFont,
    cache: HashMap<String, ParsedText>, // replace with LRU
}

pub struct Fonts {
    fonts: IntMap<u64, Font>,
}

impl Fonts {
    pub fn initialize() -> Self {
        Self {
            fonts: IntMap::default(),
        }
    }

    pub fn add_font(&mut self, id: u64, texture_id: u64, path: &str) {
        let Ok(file) = read_file(path) else {
            log::error!("couldn't load font file: {}", path);
            return;
        };
        let font = BMFont::new(Cursor::new(&file), OrdinateOrientation::BottomToTop).unwrap();
        let cache = HashMap::default();
        self.fonts.insert(
            id,
            Font {
                texture_id,
                bmfont: font,
                cache,
            },
        );
    }

    pub fn parse_text(&mut self, id: u64, text: &str) -> Option<&ParsedText> {
        let Some(font) = self.fonts.get_mut(&id) else {
            return None;
        };

        let chars = font.bmfont.parse(text);
        let parsed_text = chars.into_iter().fold(
            (Vec::with_capacity(text.len()), 0),
            |(mut char_positions, mut total_width), char| {
                let max_x = (char.screen_rect.x + char.screen_rect.width as i32) as u32;
                if total_width < max_x {
                    total_width = max_x;
                }
                char_positions.push(char);
                (char_positions, total_width)
            },
        );
        font.cache.insert(text.to_string(), parsed_text);
        font.cache.get(text)
    }

    pub fn get_texture_id(&self, id: u64) -> Option<u64> {
        self.fonts.get(&id).map(|font| font.texture_id)
    }

    pub fn get_text(&mut self, id: u64, text: &str) -> Option<&ParsedText> {
        let Some(font) = self.fonts.get_mut(&id) else {
            return None;
        };
        font.cache.get(text)
    }
}
