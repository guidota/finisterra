use engine::{
    draw::{
        text::{DrawText, ParsedText},
        Color, Position, Target,
    },
    engine::{GameEngine, TextureID},
};

use super::{Alignment, Widget};

pub struct Label {
    pub text: ParsedText,
    pub position: (u16, u16),
    pub color: Color,
    pub texture_id: TextureID,
    pub alignment: Alignment,
}

impl Label {
    pub fn new(text: ParsedText, texture_id: TextureID, color: Color) -> Self {
        Self {
            text,
            texture_id,
            color,
            position: (0, 0),
            alignment: Alignment::Center,
        }
    }

    pub fn from<E: GameEngine>(
        text: &str,
        texture_id: TextureID,
        color: Color,
        engine: &mut E,
    ) -> Self {
        let text = engine.parse_text(texture_id, text).expect("can parse");
        Self::new(text, texture_id, color)
    }

    pub fn set_text<E: GameEngine>(&mut self, text: &str, engine: &mut E) {
        let text = engine.parse_text(self.texture_id, text).expect("can parse");
        self.text = text;
    }
}

impl Widget for Label {
    fn update<E: GameEngine>(&mut self, _engine: &mut E) {}

    fn draw<E: GameEngine>(&mut self, engine: &mut E) {
        let x = self.position.0;
        let y = self.position.1 - self.text.height / 3;
        engine.draw_text(
            self.texture_id,
            DrawText {
                text: &self.text,
                position: Position::new(x, y, 1.),
                color: self.color,
            },
            Target::UI,
        )
    }
}
