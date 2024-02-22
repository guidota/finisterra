use engine::{
    draw::{image::DrawImage, Color, Position, Target},
    engine::{GameEngine, TextureID},
};

use super::Widget;

pub struct Image {
    pub position: (u16, u16),
    pub color: Color,
    pub texture_id: TextureID,

    size: (u16, u16),
    pub percent: u16,
}

impl Image {
    pub fn new(texture_id: TextureID, color: Color, position: (u16, u16)) -> Self {
        Self {
            texture_id,
            color,
            position,
            size: (0, 0),
            percent: 100,
        }
    }

    pub fn set_percent(&mut self, percent: u16) {
        if percent > 100 {
            self.percent = 100;
        } else {
            self.percent = percent;
        }
    }
}

impl Widget for Image {
    fn update<E: GameEngine>(&mut self, engine: &mut E) {
        if let Some(size) = engine.texture_dimensions(self.texture_id) {
            self.size = size;
        }
    }

    fn draw<E: GameEngine>(&mut self, engine: &mut E) {
        let x = self.position.0 - self.size.0 / 2;
        let y = self.position.1 - self.size.1 / 2;

        let width = self.size.0 * self.percent / 100;

        engine.draw_image(
            DrawImage {
                position: Position::new(x, y, 1.),
                color: self.color,
                index: self.texture_id,
                source: [0, 0, width, self.size.1],
            },
            Target::UI,
        )
    }
}
