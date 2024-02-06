use engine::{
    game::{run_game, Game},
    settings::Settings,
};
use roma::Roma;

pub fn main() {
    let settings = Settings::default();
    run_game::<ExampleGame, Roma>(settings);
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
