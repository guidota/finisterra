use std::collections::VecDeque;

use engine::{
    draw::{
        text::{DrawText, ParsedText},
        Color, Position, Target,
    },
    engine::FontID,
};

use crate::ui::{
    colors::*,
    fonts::{TAHOMA_BOLD_8_ID, TAHOMA_BOLD_8_SHADOW_ID, TAHOMA_REGULAR_8_ID},
    Widget,
};

const MAX_LINES: usize = 300;
const LINES: usize = 6;
const LINE_HEIGHT: u16 = 14;

pub struct Console {
    pub position: (u16, u16),
    pub z: f32,

    pub lines: VecDeque<Line>,
}

pub struct Line {
    text: ParsedText,
    color: Color,
    font_id: FontID,
}

impl Line {
    pub fn new<E: engine::engine::GameEngine>(
        engine: &mut E,
        text: &str,
        color: Color,
        font_id: FontID,
    ) -> Self {
        Self {
            text: engine.parse_text(font_id, text).expect("can parse"),
            color,
            font_id,
        }
    }
}

impl Console {
    pub fn initialize<E: engine::engine::GameEngine>(engine: &mut E) -> Self {
        let mut console = Self {
            position: (0, 0),
            z: 1.,
            lines: VecDeque::new(),
        };

        console.push(engine, "Test line 1", GRAY_2, TAHOMA_REGULAR_8_ID);
        console.push(engine, "Test line 2", RED, TAHOMA_BOLD_8_SHADOW_ID);
        console.push(engine, "Test line 3", YELLOW, TAHOMA_BOLD_8_ID);

        console
    }

    pub fn push<E: engine::engine::GameEngine>(
        &mut self,
        engine: &mut E,
        text: &str,
        color: Color,
        font_id: FontID,
    ) {
        let line = Line::new(engine, text, color, font_id);
        if self.lines.len() >= MAX_LINES {
            self.lines.pop_back();
        }
        self.lines.push_front(line);
    }
}

impl Widget for Console {
    fn update<E: engine::engine::GameEngine>(&mut self, _context: &mut crate::game::Context<E>) {}

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        let mut y = self.position.1 - LINE_HEIGHT;
        let x = self.position.0;
        for line in self.lines.iter().take(LINES).rev() {
            context.engine.draw_text(
                line.font_id,
                DrawText {
                    text: &line.text,
                    position: Position::new(x + line.text.total_width / 2, y, self.z),
                    color: line.color,
                },
                Target::UI,
            );

            y -= line.text.height;
        }
    }
}
