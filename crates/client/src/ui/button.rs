use engine::{
    draw::{image::DrawImage, Color, Position, Target},
    engine::{GameEngine, TextureID},
};

use crate::game::Context;

use super::{
    colors::{self, GRAY_5},
    label::Label,
    Alignment, Widget,
};

#[derive(Default)]
pub struct Button {
    pub position: (u16, u16),
    pub size: (u16, u16),
    pub color: [u8; 4],

    pub texture_id: TextureID,
    pub label: Option<Label>,

    pub state: State,
    pub selected: bool,

    pub alignment: Alignment,
}

#[derive(Default)]
pub enum State {
    #[default]
    Normal,
    Hover,
    Clicked,
    Held,
}

impl Button {
    pub fn new(
        position: (u16, u16),
        size: (u16, u16),
        texture_id: TextureID,
        label: Option<Label>,
    ) -> Self {
        Self {
            position,
            size,
            color: [255, 255, 255, 255],
            texture_id,
            label,
            state: State::Normal,
            selected: false,
            alignment: Alignment::Center,
        }
    }

    pub fn from(texture_id: TextureID) -> Self {
        let mut button = Self::new((0, 0), (0, 0), texture_id, None);
        button.color = GRAY_5;
        button
    }
}

impl Widget for Button {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Some(size) = context.engine.texture_dimensions(self.texture_id) {
            self.size = size;
        }
        let zoom = match context.engine.get_camera_zoom() {
            engine::camera::Zoom::None => 1.,
            engine::camera::Zoom::Double => 2.,
        };
        let mouse_position = context.engine.mouse_position();
        let (x, y) = (
            (mouse_position.x / zoom) as u16,
            (mouse_position.y / zoom) as u16,
        );

        let (x_start, y_start, x_end, y_end) = self.rect();

        if x > x_start && x < x_end && y > y_start && y < y_end {
            if context.engine.mouse_released() {
                self.state = State::Clicked;
            } else if context.engine.mouse_held() {
                self.state = State::Held;
            } else {
                self.state = State::Hover;
            }
        } else {
            self.state = State::Normal;
        }
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let button_rect = self.rect();
        let (x, y, _, _) = button_rect;
        context.engine.draw_image(
            DrawImage {
                position: Position::new(x, y, 0.),
                index: self.texture_id,
                color: self.color(),
                ..Default::default()
            },
            Target::UI,
        );

        if let Some(label) = self.label.as_mut() {
            label.position = (
                (button_rect.0 + button_rect.2) / 2,
                (button_rect.1 + button_rect.3) / 2,
            );
            label.draw(context);
        }
    }
}

impl Button {
    fn color(&self) -> Color {
        if self.selected {
            colors::tint(self.color, 0.8)
        } else {
            match self.state {
                State::Normal => self.color,
                State::Hover => colors::tint(self.color, 0.15),
                State::Clicked => colors::shade(self.color, 0.7),
                State::Held => colors::shade(self.color, 0.8),
            }
        }
    }

    fn rect(&self) -> (u16, u16, u16, u16) {
        match self.alignment {
            Alignment::Left => (
                self.position.0,
                self.position.1,
                self.position.0 + self.size.0,
                self.position.1 + self.size.1,
            ),
            Alignment::Center => (
                self.position.0 - self.size.0 / 2,
                self.position.1,
                self.position.0 + self.size.0 / 2,
                self.position.1 + self.size.1,
            ),
            Alignment::Right => (
                self.position.0 - self.size.0,
                self.position.1,
                self.position.0,
                self.position.1 + self.size.1,
            ),
        }
    }

    pub fn clicked(&self) -> bool {
        matches!(self.state, State::Clicked)
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn unselect(&mut self) {
        self.selected = false;
    }
}

#[derive(Default)]
pub struct ButtonBuilder {
    button: Button,
}

impl ButtonBuilder {
    pub fn new() -> ButtonBuilder {
        Self {
            button: Button::default(),
        }
    }

    pub fn texture_id(mut self, texture_id: TextureID) -> Self {
        self.button.texture_id = texture_id;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.button.color = color;
        self
    }

    pub fn size(mut self, size: (u16, u16)) -> Self {
        self.button.size = size;
        self
    }

    pub fn position(mut self, position: (u16, u16)) -> Self {
        self.button.position = position;
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.button.label = Some(label);
        self
    }

    pub fn build(self) -> Button {
        self.button
    }
}
