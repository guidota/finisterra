use std::sync::mpsc::{channel, Receiver, Sender};

use engine::{engine::GameEngine, game::Game};
use tracing::info;

use crate::{
    networking::connection::ConnectionState,
    resources::Resources,
    screens::{home::HomeScreen, GameScreen, Screen},
    ui::{self},
};

pub struct Finisterra {
    resources: Resources,

    connection: ConnectionState,

    screen: Screen,
    screen_transition: (Sender<Screen>, Receiver<Screen>),
}

pub struct Context<'tick, E: GameEngine> {
    pub engine: &'tick mut E,
    pub screen_transition_sender: &'tick Sender<Screen>,
    pub connection: &'tick mut ConnectionState,
    pub resources: &'tick Resources,
}

impl Game for Finisterra {
    fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        ui::load(engine);
        let resources = Resources::load(engine);
        let screen_transition = channel();
        let mut connection = ConnectionState::new("https://[::1]:7666");
        let mut context = Context {
            screen_transition_sender: &screen_transition.0,
            connection: &mut connection,
            resources: &resources,
            engine,
        };
        let x = HomeScreen::new(&mut context);
        Self {
            resources,
            screen: Screen::Home(Box::new(x)),
            connection,
            screen_transition,
        }
    }

    fn tick<E: GameEngine>(&mut self, engine: &mut E) {
        let delta = engine.get_delta();
        self.connection.update(delta);
        self.connection.draw_state(engine);

        let mut context = Context {
            screen_transition_sender: &self.screen_transition.0,
            connection: &mut self.connection,
            resources: &self.resources,
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
