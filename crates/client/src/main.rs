use engine::game::run_game;
use game::Finisterra;
use roma::Roma;

mod game;
pub mod networking;
pub mod resources;
pub mod screens;
pub mod ui;

#[tokio::main]
async fn main() {
    let settings = engine::settings::Settings {
        width: 920,
        height: 540,
        title: "Finisterra".to_string(),
        vsync: true,
    };

    run_game::<Finisterra, Roma>(settings).await;
}
