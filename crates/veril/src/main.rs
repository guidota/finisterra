use engine::{
    game::{run_game, Game},
    settings::Settings,
};
use veril::Veril;

pub fn main() {
    let settings = Settings::default();
    run_game::<ExampleGame, Veril>(settings);
}

struct ExampleGame {}

impl Game for ExampleGame {
    fn initialize<E: engine::engine::GameEngine>(_engine: &mut E) -> Self {
        Self {}
    }

    fn tick<E: engine::engine::GameEngine>(&mut self, _engine: &mut E) {
        println!("tick");
    }
}
