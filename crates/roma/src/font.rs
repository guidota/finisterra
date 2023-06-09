use std::io::Cursor;

use bmfont::{BMFont, OrdinateOrientation};
use rustc_hash::FxHashMap;

use crate::{renderer::texture, roma::get_state, Rect};

type ParseResult = Vec<(i32, i32, Rect)>;

pub struct Fonts {
    pub font: BMFont,
    pub cache_glyphs: FxHashMap<String, (ParseResult, usize)>,
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

    pub fn parse(&mut self, text: &str) -> &(ParseResult, usize) {
        self.cache_glyphs
            .entry(text.to_string())
            .or_insert_with(|| {
                let Ok(chars) = self.font.parse(text) else {
                    return (vec![], 0);
                };

                chars.into_iter().fold(
                    (Vec::with_capacity(text.len()), 0),
                    |(mut data, mut total_width), char| {
                        let x = char.screen_rect.x;
                        let y = char.screen_rect.y;
                        let source = Rect {
                            x: char.page_rect.x as usize,
                            y: char.page_rect.y as usize,
                            w: char.screen_rect.width as usize,
                            h: char.screen_rect.height as usize,
                        };
                        total_width += source.w;
                        data.push((x, y, source));
                        (data, total_width)
                    },
                )
            })
    }

    pub fn create_font_texture() -> (usize, texture::Texture) {
        let state = get_state();
        let image = image::load_from_memory(include_bytes!("fonts/shadowed-font.png")).unwrap();
        let font_texture =
            texture::Texture::from_image(&state.device, &state.queue, &image, Some("font"));

        (RESERVED_ID, font_texture)
    }
}
