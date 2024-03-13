use engine::{
    draw::{
        text::{DrawText, ParsedText},
        Color, Position, Target,
    },
    engine::{FontID, GameEngine, TextureID},
};

use crate::game::Context;

use super::{Alignment, Widget};

pub struct Label {
    pub parsed_text: ParsedText,
    pub text: String,
    pub position: (u16, u16),
    pub z: f32,
    pub color: Color,
    pub font_id: FontID,
    pub alignment: Alignment,
}

impl Label {
    pub fn new(text: String, parsed_text: ParsedText, texture_id: TextureID, color: Color) -> Self {
        Self {
            text,
            parsed_text,
            font_id: texture_id,
            color,
            position: (0, 0),
            alignment: Alignment::Center,
            z: 1.,
        }
    }

    pub fn from<E: GameEngine>(
        text: &str,
        texture_id: FontID,
        color: Color,
        engine: &mut E,
    ) -> Self {
        let parsed_text = engine.parse_text(texture_id, text).expect("can parse");
        Self::new(text.to_string(), parsed_text, texture_id, color)
    }

    pub fn set_text<E: GameEngine>(&mut self, text: &str, engine: &mut E) {
        if self.text != text {
            let parsed_text = engine.parse_text(self.font_id, text).expect("can parse");
            self.parsed_text = parsed_text;
        }
    }
}

impl Widget for Label {
    fn update<E: GameEngine>(&mut self, _context: &mut Context<E>) {}

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let x = match self.alignment {
            Alignment::Right => self.position.0 - self.parsed_text.total_width / 2,
            Alignment::Center => self.position.0,
            Alignment::Left => self.position.0 + self.parsed_text.total_width / 2,
        };

        let y = self.position.1 - self.parsed_text.height / 3;
        context.engine.draw_text(
            self.font_id,
            DrawText {
                text: &self.parsed_text,
                position: Position::new(x, y, self.z),
                color: self.color,
            },
            Target::UI,
        )
    }
}
