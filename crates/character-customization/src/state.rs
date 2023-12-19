use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use roma::{
    get_delta,
    ui::{self, ManagedTextureId},
};

use crate::{
    app::{App, LoadingStage},
    ui::chooser::chooser,
};

pub enum State {
    Loading {
        stage: LoadingStage,
        rx: Receiver<LoadingStage>,
        background: Option<ManagedTextureId>,
        size: (usize, usize),
    },
    Started {
        app: Box<App>,
        size: (usize, usize),
        background: ManagedTextureId,
    },
}

impl State {
    pub fn init() -> State {
        // create channel
        let (tx, rx): (Sender<LoadingStage>, Receiver<LoadingStage>) = channel();

        // spawn thread loading assets
        thread::spawn(move || {
            App::load(tx);
        });

        State::Loading {
            stage: LoadingStage::Init,
            rx,
            background: None,
            size: (800, 600),
        }
    }

    pub fn update(&mut self) {
        match self {
            State::Loading {
                rx,
                stage,
                size,
                background,
            } => {
                if let Ok(update) = rx.try_recv() {
                    match update {
                        LoadingStage::Finish(app) => {
                            *self = State::Started {
                                app,
                                size: *size,
                                background: background.unwrap(),
                            }
                        }
                        LoadingStage::Background(texture_id) => *background = Some(texture_id),
                        _ => *stage = update,
                    }
                }
            }
            State::Started { app, .. } => {
                let delta = get_delta();
                app.update(delta);
            }
        }
    }

    pub fn draw(&mut self) {
        match self {
            State::Loading {
                stage,
                background,
                size,
                ..
            } => {
                if let Some(background) = background {
                    ui::image(*background, ui::Vec2::new(size.0 as f32, size.1 as f32));
                }
                let text = format!("{}", stage);
                let mut row = ui::widgets::List::row();
                row.main_axis_alignment = ui::MainAxisAlignment::Center;
                row.cross_axis_alignment = ui::CrossAxisAlignment::End;
                row.show(|| {
                    ui::text(12., text);
                });
            }
            State::Started {
                background,
                size,
                app,
                ..
            } => {
                ui::image(*background, [size.0 as f32, size.1 as f32]);
                ui::column(|| {
                    ui::expanded(|| {});
                    let mut row = ui::widgets::List::row();
                    row.main_axis_alignment = ui::MainAxisAlignment::Center;
                    row.cross_axis_alignment = ui::CrossAxisAlignment::End;

                    row.show(|| {
                        chooser("Direction", app.character.animator.direction, |direction| {
                            app.character.change_direction(direction)
                        });
                        chooser("Animation", app.character.animator.animation, |animation| {
                            app.character.change_animation(animation)
                        });
                    });
                });

                app.draw(*size);
            }
        }
    }

    pub fn resize(&mut self, new_size: (usize, usize)) {
        match self {
            State::Loading { size, .. } => *size = new_size,
            State::Started { size, .. } => *size = new_size,
        }
    }
}
