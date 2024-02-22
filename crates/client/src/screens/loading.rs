use std::{
    fmt::Display,
    sync::mpsc::{channel, Receiver},
    thread,
};

use engine::{
    draw::{text::DrawText, Position, Target},
    engine::GameEngine,
};

use crate::{game::Context, resources::Resources, ui::fonts::TAHOMA_BOLD_8_SHADOW_ID};

use super::{home::HomeScreen, GameScreen, Screen};

pub struct LoadingScreen {
    state: LoadingState,

    state_updates_receiver: Receiver<LoadingState>,
}

#[derive(Debug)]
pub enum LoadingState {
    Init,
    LoadingBackground,
    LoadingBodies,
    LoadingHeads,
    LoadingHelmets,
    LoadingShields,
    LoadingWeapons,
    LoadingArmors,
    Finish { resources: Resources },
}

impl LoadingScreen {
    pub fn new() -> Self {
        let (state_updates_sender, state_updates_receiver) = channel();
        thread::spawn(move || {
            Resources::load(state_updates_sender);
        });
        Self {
            state: LoadingState::Init,
            state_updates_receiver,
        }
    }
}

impl Default for LoadingScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl GameScreen for LoadingScreen {
    fn update<E: GameEngine>(&mut self, context: &mut Context<E>) {
        if let Ok(loading_state) = self.state_updates_receiver.try_recv() {
            match loading_state {
                LoadingState::Finish { resources } => {
                    context
                        .screen_transition_sender
                        .send(Screen::Home(Box::new(HomeScreen::new(
                            resources,
                            context.engine,
                        ))))
                        .expect("poisoned");
                }
                _ => {
                    self.state = loading_state;
                }
            }
        }
        let window_size = context.engine.get_window_size();
        let height = window_size.height as f32;
        let width = window_size.width as f32;
        let viewport = engine::camera::Viewport {
            x: 0.,
            y: 0.,
            width,
            height,
        };
        context.engine.set_world_camera_viewport(viewport);
        context.engine.set_ui_camera_viewport(viewport);
    }

    fn draw<E: GameEngine>(&mut self, context: &mut Context<E>) {
        let text = context
            .engine
            .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format!("{}", self.state));
        context.engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &text.unwrap(),
                position: Position {
                    x: 10,
                    y: 10,
                    z: 1.,
                },
                color: [255, 255, 255, 255],
            },
            Target::UI,
        );
    }
}

impl Display for LoadingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LoadingState::Init => "Initializing...",
            LoadingState::LoadingBackground => "Loading background...",
            LoadingState::LoadingBodies => "Loading bodies...",
            LoadingState::LoadingHeads => "Loading heads...",
            LoadingState::LoadingHelmets => "Loading helmets...",
            LoadingState::LoadingShields => "Loading shields...",
            LoadingState::LoadingWeapons => "Loading weapons...",
            LoadingState::LoadingArmors => "Loading armors...",
            LoadingState::Finish { .. } => "Resources loaded!",
        })
    }
}
