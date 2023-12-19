use roma::{run_game, Settings};
use state::State;

mod app;
mod state;
mod ui;

fn main() {
    let settings = settings();

    run_game(settings, init, game_loop, resize_handler)
}

fn settings() -> Settings {
    Settings {
        width: 800,
        height: 600,
        title: "Finisterra - Character customization".to_string(),
        present_mode: roma::PresentMode::AutoVsync,
    }
}

fn init() -> State {
    State::init()
}

fn game_loop(state: &mut State) {
    state.update();
    state.draw();
}

fn resize_handler(state: &mut State, new_size: (usize, usize)) {
    state.resize(new_size)
}
