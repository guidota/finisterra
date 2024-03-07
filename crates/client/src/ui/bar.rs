use std::time::Duration;

use engine::{
    draw::Color,
    engine::{GameEngine, TextureID},
};
use interpolation::lerp;

use crate::{game::Context, ui::Widget};

use super::{colors::*, fonts::*, image::Image, label::Label, textures::*};

pub struct Bar {
    pub label: Label,
    pub image: Image,

    min: u64,
    max: u64,

    target: u16,
    interpolation_time: Duration,
}

impl Bar {
    pub fn new<E: GameEngine>(engine: &mut E, color: Color) -> Self {
        let label = Label::from("", TAHOMA_BOLD_8_SHADOW_ID, GRAY_6, engine);

        let mut image = Image::new(BAR_ID, color, (0, 0));
        image.percent = 100;
        Self {
            label,
            image,
            min: 0,
            max: 0,
            target: 0,
            interpolation_time: Duration::ZERO,
        }
    }

    pub fn with<E: GameEngine>(engine: &mut E, texture_id: TextureID, text_color: Color) -> Self {
        let label = Label::from("", TAHOMA_BOLD_8_SHADOW_ID, text_color, engine);

        let mut image = Image::new(texture_id, [255, 255, 255, 120], (0, 0));
        image.percent = 100;
        Self {
            label,
            image,
            min: 100,
            max: 100,
            target: 100,
            interpolation_time: Duration::ZERO,
        }
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.label.position = (x, y);
        self.image.position = (x, y);
    }

    pub fn set_values<E: GameEngine, T: Into<u64>>(&mut self, engine: &mut E, (min, max): (T, T)) {
        let min = min.into();
        let max = max.into();
        if min != self.min || max != self.max {
            self.min = min;
            self.max = max;
            self.label.parsed_text = engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format! {"{min}/{max}"})
                .expect("can parse");

            let percent = ((min as f32 / max as f32) * 100.) as u16;
            self.target = percent;
            self.interpolation_time = Duration::ZERO;
        }
    }

    pub fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.image.update(context);
        const INTERPOLIATION_DURATION: Duration = Duration::from_millis(250);
        if self.target != self.image.percent && self.interpolation_time < INTERPOLIATION_DURATION {
            let delta = context.engine.get_delta();
            self.interpolation_time += delta;

            let time_percent = self.interpolation_time.as_millis() as f32
                / INTERPOLIATION_DURATION.as_millis() as f32;
            let percent = lerp(&self.image.percent, &self.target, &time_percent);
            self.image.set_percent(percent);
        }
    }

    pub fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        self.image.draw(context);
        self.label.draw(context);
    }
}
