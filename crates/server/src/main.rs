use anyhow::Result;
use finisterra::Finisterra;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

mod accounts;
mod finisterra;
mod server;
mod world;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    Finisterra::initialize().await?.run().await?;

    Ok(())
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
