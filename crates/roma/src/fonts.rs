use engine::draw::text::ParsedText;
use std::io::Cursor;

use bmfont::{BMFont, OrdinateOrientation};
use nohash_hasher::IntMap;

use crate::files::read_file;

pub struct Font {
    texture_id: u64,
    bmfont: bmfont::BMFont,
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
        self.fonts.insert(
            id,
            Font {
                texture_id,
                bmfont: font,
            },
        );
    }

    pub fn parse_text(&mut self, id: u64, text: &str) -> Option<ParsedText> {
        let Some(font) = self.fonts.get_mut(&id) else {
            return None;
        };

        let chars = font.bmfont.parse(text);
        Some(chars.into_iter().fold(
            ParsedText {
                chars: vec![],
                total_width: 0,
            },
            |ParsedText {
                 mut chars,
                 mut total_width,
             },
             char| {
                let max_x = (char.screen_rect.x + char.screen_rect.width as i32) as u32;
                if total_width < max_x {
                    total_width = max_x;
                }
                chars.push(char);
                ParsedText { chars, total_width }
            },
        ))
    }

    pub fn get_texture_id(&self, id: u64) -> Option<u64> {
        self.fonts.get(&id).map(|font| font.texture_id)
    }
}
