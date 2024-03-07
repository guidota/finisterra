use std::{ops::AddAssign, time::Duration};

use engine::{
    draw::{
        image::DrawImage,
        text::{DrawText, ParsedText},
        Position, Target,
    },
    engine::{GameEngine, TextureID},
    input::keyboard::{Key, NamedKey},
    CursorIcon,
};

use crate::game::Context;

use super::Widget;

const TIME_TO_BLINK: Duration = Duration::from_millis(200);

pub struct InputField {
    font_color: [u8; 4],
    background_color: [u8; 4],
    pub position: (u16, u16),
    size: (u16, u16),
    font_texture_id: TextureID,
    text: String,
    parsed_text: ParsedText,

    blinking_cursor: ParsedText,
    blinking_transparency: u8,
    blinking_time: Duration,

    background_texture_id: TextureID,

    char_position: usize,
    pub focused: bool,
    pub obfuscate: bool,

    mouse_in: bool,
}

impl InputField {
    pub fn new<E: GameEngine>(
        font_color: [u8; 4],
        background_color: [u8; 4],
        position: (u16, u16),
        size: (u16, u16),
        font_texture_id: TextureID,
        background_texture_id: TextureID,
        context: &mut Context<E>,
    ) -> Self {
        let text = "".to_string();
        let parsed_text = context.engine.parse_text(font_texture_id, "").unwrap();
        let blinking_cursor = context.engine.parse_text(font_texture_id, "|").unwrap();

        Self {
            font_color,
            background_color,
            position,
            size,
            font_texture_id,
            background_texture_id,
            text,
            parsed_text,
            focused: false,
            obfuscate: false,
            blinking_cursor,
            blinking_transparency: 0,
            blinking_time: Duration::ZERO,
            char_position: 0,
            mouse_in: false,
        }
    }

    fn update_mouse<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let zoom = match context.engine.get_camera_zoom() {
            engine::camera::Zoom::None => 1.,
            engine::camera::Zoom::Double => 2.,
        };
        let mouse_position = context.engine.mouse_position();
        let (x, y) = (
            (mouse_position.x / zoom) as u16,
            (mouse_position.y / zoom) as u16,
        );

        let (x_start, y_start, x_end, y_end) = (
            self.position.0,
            self.position.1,
            self.position.0 + self.size.0,
            self.position.1 + self.size.1,
        );

        let mouse_in = x > x_start && x < x_end && y > y_start && y < y_end;

        match (self.mouse_in, mouse_in) {
            (true, false) => self.on_mouse_exit(context),
            (false, true) => self.on_mouse_enter(context),
            _ => {}
        }

        if context.engine.mouse_clicked() {
            self.focused = mouse_in;
        }
    }

    fn update_pressed_keys<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if !self.focused {
            return;
        }

        let pressed_keys = context.engine.pressed_keys();
        for key in pressed_keys.iter() {
            self.process_key(key, context);
        }

        let held_keys = context.engine.held_keys();
        for key in held_keys {
            self.process_key(&key, context);
        }
    }

    fn update_blinking_cursor<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if self.focused {
            let delta = context.engine.get_delta();
            if self.blinking_time.ge(&TIME_TO_BLINK) {
                self.blinking_time = Duration::ZERO;
                if self.blinking_transparency > 0 {
                    self.blinking_transparency = 0;
                } else {
                    self.blinking_transparency = 255;
                }
            } else {
                self.blinking_time.add_assign(delta);
            }
        }
    }

    fn process_key<E: GameEngine>(&mut self, current_key: &Key, context: &mut Context<E>) {
        match current_key {
            Key::Named(NamedKey::Backspace) => {
                if !self.text.is_empty() {
                    self.text.remove(self.char_position - 1);
                    if self.char_position >= 1 {
                        self.char_position -= 1;
                    }
                    self.prepare_text(context);
                }
            }
            Key::Named(NamedKey::ArrowLeft) => {
                if self.char_position >= 1 {
                    self.char_position -= 1;
                }
            }
            Key::Named(NamedKey::ArrowRight) => {
                self.char_position = std::cmp::min(self.text.len(), self.char_position + 1);
            }
            Key::Named(NamedKey::Space) => {
                self.text.insert(self.char_position, ' ');
                self.char_position = std::cmp::min(self.text.len(), self.char_position + 1);
                self.prepare_text(context);
            }
            Key::Character(ref char) => {
                self.text.insert_str(self.char_position, char);
                self.char_position = std::cmp::min(self.text.len(), self.char_position + 1);
                self.prepare_text(context);
            }
            _ => {}
        }
    }

    fn prepare_text<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if self.obfuscate {
            let mut text = String::new();
            for _ in 0..self.text.len() {
                text += "*";
            }
            self.parsed_text = context
                .engine
                .parse_text(self.font_texture_id, &text)
                .expect("can parse text");
        } else {
            self.parsed_text = context
                .engine
                .parse_text(self.font_texture_id, &self.text)
                .expect("can parse text");
        }
    }

    fn on_mouse_enter<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.mouse_in = true;
        context.engine.set_mouse_cursor(CursorIcon::Text);
    }

    fn on_mouse_exit<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.mouse_in = false;
        // TODO: handle set prev mouse cursor
        context.engine.set_mouse_cursor(CursorIcon::Default);
    }
}

impl Widget for InputField {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.update_mouse(context);
        self.update_pressed_keys(context);
        self.update_blinking_cursor(context);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        // draw background
        context.engine.draw_image(
            DrawImage {
                position: Position::new(self.position.0 - self.size.0 / 2, self.position.1, 0.9),
                color: self.background_color,
                index: self.background_texture_id,
                ..Default::default()
            },
            Target::UI,
        );

        // draw text
        let center = self.position.0;
        let y = self.position.1 + self.size.1 / 2 - 6;
        context.engine.draw_text(
            self.font_texture_id,
            DrawText {
                text: &self.parsed_text,
                position: Position::new(center, y, 1.),
                color: self.font_color,
            },
            Target::UI,
        );

        // if focused draw blinking cursor
        if self.focused {
            let mut color = self.font_color;
            color[3] = self.blinking_transparency;

            let text_start = center - (self.parsed_text.total_width / 2);
            let substring = {
                if self.obfuscate {
                    let mut text = String::new();
                    for _ in 0..self.text.len() {
                        text += "*";
                    }
                    text
                } else {
                    self.text[..self.char_position].to_string()
                }
            };
            let text_width = context
                .engine
                .parse_text(self.font_texture_id, &substring)
                .unwrap()
                .total_width;

            context.engine.draw_text(
                self.font_texture_id,
                DrawText {
                    text: &self.blinking_cursor,
                    position: Position::new(text_start + text_width + 1, y + 1, 1.),
                    color,
                },
                Target::UI,
            );
        }
    }
}

impl InputField {
    pub fn text(&self) -> &str {
        &self.text
    }
}
