use engine::game::run_game;
use game::Finisterra;
use roma::Roma;

mod argentum;
pub mod connection;
mod game;
pub mod maps;
pub mod resources;
pub mod screens;
pub mod texture;
pub mod ui;

#[tokio::main]
async fn main() {
    let settings = engine::settings::Settings {
        width: 800,
        height: 540,
        title: "finisterra".to_string(),
        vsync: true,
    };

    run_game::<Finisterra, Roma>(settings).await;
}
