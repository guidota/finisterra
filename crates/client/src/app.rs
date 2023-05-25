use std::sync::Arc;

use crate::game::Game;

use self::resources::Resources;

pub mod resources;
pub mod window;

enum State {
    Welcome,
    Game,
}

pub struct App {
    state: State,
    _resources: Arc<Resources>,
    game: Game,
}

impl App {
    pub async fn new() -> Self {
        let resources = Resources::load().await;
        let resources = Arc::new(resources);
        let game = Game::new(resources.clone());

        Self {
            state: State::Welcome,
            _resources: resources,
            game,
        }
    }

    pub async fn update(&mut self) {
        match &self.state {
            State::Welcome => {
                self.state = State::Game;
            }
            State::Game => {
                self.game.update().await;
            }
        }
    }

    pub async fn render(&mut self) {
        match &self.state {
            State::Welcome => {}
            State::Game => {
                self.game.render().await;
            }
        }
    }
}
