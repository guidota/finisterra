use std::{borrow::Cow, fmt::Display};

use lorenzo::character::{animation::CharacterAnimation, direction::Direction};
use roma::ui::{
    self,
    widgets::{Button, ButtonWidget, Pad},
    Response,
};

pub fn chooser<T: Chooser + Display>(title: &'static str, variant: T, mut update: impl FnMut(T)) {
    ui::pad(ui::widgets::Pad::all(15.), || {
        let width = 100.;
        let height = 35.;
        let padding = 4.;

        let mut rect = ui::widgets::RoundRect::new(4.);
        rect.color = ui::Color::BLACK.with_alpha(0.7);
        rect.min_size = ui::Vec2::new(width + padding * 2., height + padding * 2. + 4.);
        rect.show();

        ui::pad(ui::widgets::Pad::all(padding), || {
            ui::column(|| {
                ui::constrained(
                    ui::Constraints::loose(ui::Vec2::new(width, height / 2.)),
                    || {
                        ui::row(|| {
                            ui::expanded(|| {
                                ui::center(|| {
                                    ui::text(14., title);
                                });
                            });
                        });
                    },
                );
                ui::constrained(
                    ui::Constraints::loose(ui::Vec2::new(width, height / 2.)),
                    || {
                        ui::row(|| {
                            if button("<").clicked {
                                update(variant.prev());
                            };
                            ui::expanded(|| {
                                ui::center(|| {
                                    ui::text(12., format!("{variant}"));
                                });
                            });
                            if button(">").clicked {
                                update(variant.next());
                            }
                        });
                    },
                );
            });
        });
    });
}

fn button(text: impl Into<Cow<'static, str>>) -> Response<ButtonWidget> {
    let mut button = Button::styled(text.into());
    button.border_radius = 2.;
    button.padding = Pad::balanced(6., 2.);
    button.show()
}

pub trait Chooser {
    fn prev(&self) -> Self;
    fn next(&self) -> Self;
}

impl Chooser for CharacterAnimation {
    fn prev(&self) -> Self {
        match self {
            CharacterAnimation::Idle => CharacterAnimation::Walk,
            CharacterAnimation::Walk => CharacterAnimation::Idle,
            CharacterAnimation::Attack => CharacterAnimation::Walk,
            CharacterAnimation::Defend => CharacterAnimation::Attack,
            CharacterAnimation::Die => CharacterAnimation::Defend,
        }
    }

    fn next(&self) -> Self {
        match self {
            CharacterAnimation::Idle => CharacterAnimation::Walk,
            CharacterAnimation::Walk => CharacterAnimation::Idle,
            CharacterAnimation::Attack => CharacterAnimation::Defend,
            CharacterAnimation::Defend => CharacterAnimation::Die,
            CharacterAnimation::Die => CharacterAnimation::Idle,
        }
    }
}

impl Chooser for Direction {
    fn prev(&self) -> Self {
        match self {
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::West => Direction::North,
        }
    }

    fn next(&self) -> Self {
        match self {
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
        }
    }
}
