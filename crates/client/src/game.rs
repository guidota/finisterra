use std::sync::mpsc::{channel, Receiver, Sender};

use engine::{engine::GameEngine, game::Game};

use crate::{
    connection::ConnectionState,
    maps::Maps,
    resources::Resources,
    screens::{home::HomeScreen, GameScreen, Screen},
    ui::fonts::Fonts,
};

pub struct Finisterra {
    resources: Resources,
    maps: Maps,

    connection: ConnectionState,

    screen: Screen,
    screen_transition: (Sender<Screen>, Receiver<Screen>),
}

pub struct Context<'tick, E: GameEngine> {
    pub engine: &'tick mut E,
    pub screen_transition_sender: &'tick Sender<Screen>,
    pub connection: &'tick mut ConnectionState,
    pub resources: &'tick Resources,
    pub maps: &'tick mut Maps,
}

impl Game for Finisterra {
    fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        let resources = Resources::load(engine);
        let mut maps = Maps::initialize("assets/finisterra/maps/");
        Fonts::load(engine);
        let screen_transition = channel();
        let mut connection = ConnectionState::new("https://[::1]:7666", engine);
        let mut context = Context {
            screen_transition_sender: &screen_transition.0,
            connection: &mut connection,
            resources: &resources,
            maps: &mut maps,
            engine,
        };
        let home_screen = HomeScreen::new(&mut context);
        Self {
            resources,
            screen: Screen::Home(Box::new(home_screen)),
            connection,
            screen_transition,
            maps,
        }
    }

    fn tick<E: GameEngine>(&mut self, engine: &mut E) {
        self.connection.update(engine);
        self.connection.draw_state(engine);

        let mut context = Context {
            screen_transition_sender: &self.screen_transition.0,
            connection: &mut self.connection,
            resources: &mut self.resources,
            maps: &mut self.maps,
            engine,
        };
        self.screen.update(&mut context);
        self.screen.draw(&mut context);

        if let Ok(screen) = self.screen_transition.1.try_recv() {
            self.screen = screen;
        }
    }
}

impl Drop for Finisterra {
    fn drop(&mut self) {
        self.connection.close();
    }
}
