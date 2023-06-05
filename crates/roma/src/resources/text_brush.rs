use glyph_brush::{
    ab_glyph::FontArc, BrushError, DefaultSectionHasher, Extra, FontId, GlyphBrushBuilder,
    Rectangle, Section,
};

use crate::render::vertex::Vertex;

pub struct TextBrush {
    inner: glyph_brush::GlyphBrush<Vec<Vertex>, Extra, FontArc, DefaultSectionHasher>,
    frame_instructions: Vec<(FontId, Vec<Vertex>)>,
}

impl TextBrush {
    pub fn tahoma() -> Self {
        let tahoma = FontArc::try_from_slice(include_bytes!("./fonts/Tahoma-Bold.otf")).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(tahoma).build();

        Self {
            inner: glyph_brush,
            frame_instructions: Vec::new(),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.inner.texture_dimensions()
    }

    pub fn queue(&mut self, section: Section) {
        self.inner.queue(section);
    }

    pub fn process_queue(
        &mut self,
        update_texture: impl Fn(Rectangle<u32>, &[u8]),
        recreate_texture: impl Fn((u32, u32)),
    ) {
        loop {
            match self.inner.process_queued(
                |size, data| {
                    update_texture(size, data);
                },
                to_vertex,
            ) {
                Ok(action) => {
                    break match action {
                        glyph_brush::BrushAction::Draw(vertices) => {
                            self.frame_instructions.clear();
                            self.frame_instructions
                                .push((FontId(0), vertices.into_iter().flatten().collect()));
                        }
                        glyph_brush::BrushAction::ReDraw => {}
                    };
                }
                Err(BrushError::TextureTooSmall { suggested }) => {
                    println!("TextureTooSmall");
                    recreate_texture(suggested);
                    self.inner.resize_texture(suggested.0, suggested.1);
                }
            }
        }
    }

    pub fn get_instructions(&mut self) -> &mut Vec<(FontId, Vec<Vertex>)> {
        &mut self.frame_instructions
    }
}

fn to_vertex(glyph_vertex: glyph_brush::GlyphVertex) -> Vec<Vertex> {
    let glyph_brush::GlyphVertex {
        tex_coords,
        pixel_coords,
        extra,
        ..
    } = glyph_vertex;
    // let height =  0;pixel_coords.max.y - pixel_coords.min.y;
    // let y = pixel_coords.max.y;
    let points = [
        [pixel_coords.min.x, pixel_coords.min.y, extra.z],
        [pixel_coords.max.x, pixel_coords.min.y, extra.z],
        [pixel_coords.max.x, pixel_coords.max.y, extra.z],
        [pixel_coords.min.x, pixel_coords.max.y, extra.z],
    ];

    let tex_coords = [
        [tex_coords.min.x, tex_coords.max.y],
        [tex_coords.max.x, tex_coords.max.y],
        [tex_coords.max.x, tex_coords.min.y],
        [tex_coords.min.x, tex_coords.min.y],
    ];

    let mut vertices = Vec::with_capacity(4);
    for i in 0..4 {
        let vertex = Vertex {
            position: points[i],
            tex_coords: tex_coords[i],
            color: extra.color,
        };
        vertices.push(vertex);
    }

    vertices
}
