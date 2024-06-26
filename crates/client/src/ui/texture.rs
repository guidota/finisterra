use engine::{
    draw::{image::DrawImage, Color, Position, Target},
    engine::{GameEngine, TextureID},
};

use crate::game::Context;

use super::Widget;

pub struct Texture {
    pub position: (u16, u16),
    pub color: Color,
    pub texture_id: TextureID,

    pub size: (u16, u16),
    pub percent: u16,
}

impl Texture {
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

impl Widget for Texture {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(size) = context.engine.texture_dimensions(self.texture_id) {
            self.size = size;
        }
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let x = self.position.0 - self.size.0 / 2;
        let y = self.position.1 - self.size.1 / 2;

        let width = self.size.0 * self.percent / 100;

        context.engine.draw_image(
            DrawImage {
                position: Position::new(x, y, 0.99),
                color: self.color,
                index: self.texture_id,
                source: [0, 0, width, self.size.1],
            },
            Target::UI,
        )
    }
}
