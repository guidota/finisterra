use engine::game::run_game;
use game::Finisterra;
use roma::Roma;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

mod game;
pub mod networking;
pub mod resources;
pub mod screens;
pub mod ui;

#[tokio::main]
async fn main() {
    init_logging();

    let settings = engine::settings::Settings {
        width: 920,
        height: 540,
        title: "Finisterra".to_string(),
        vsync: true,
    };

    run_game::<Finisterra, Roma>(settings).await;
}

pub fn init_logging() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_env_filter(env_filter)
        .init();
}
