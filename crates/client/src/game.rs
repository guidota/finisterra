use std::sync::mpsc::{channel, Receiver, Sender};

use engine::{engine::GameEngine, game::Game};
use tracing::info;

use crate::{
    networking::connection::ConnectionState,
    screens::{loading::LoadingScreen, GameScreen, Screen},
    ui::{self},
};

pub struct Finisterra {
    connection: ConnectionState,

    screen: Screen,
    screen_transition: (Sender<Screen>, Receiver<Screen>),
}

pub struct Context<'tick, E: GameEngine> {
    pub engine: &'tick mut E,
    pub screen_transition_sender: &'tick Sender<Screen>,
    pub connection: &'tick mut ConnectionState,
}

impl Game for Finisterra {
    fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        ui::load(engine);

        Self {
            screen: Screen::Loading(Box::new(LoadingScreen::new())),
            connection: ConnectionState::new("https://[::1]:7666"),
            screen_transition: channel(),
        }
    }

    fn tick<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        self.connection.update(delta);
        self.connection.draw_state(engine);

        let mut context = Context {
            screen_transition_sender: &self.screen_transition.0,
            connection: &mut self.connection,
            engine,
        };
        self.screen.update(&mut context);
        self.screen.draw(&mut context);

        if let Ok(screen) = self.screen_transition.1.try_recv() {
            info!("Transitioning to screen: {screen}");
            self.screen = screen;
        }
    }
}

impl Drop for Finisterra {
    fn drop(&mut self) {
        self.connection.close();
    }
}