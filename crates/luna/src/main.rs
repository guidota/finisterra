use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
    Router, routing::get,
};
use axum::extract::Path;

use database::Database;

struct StateDB {
    database: Database,
}

#[tokio::main]
async fn main() -> Result<()> {
    let database = Database::initialize().await?;

    let state = Arc::new(StateDB { database });

    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/accounts", get(accounts))
        .route("/accounts/:mail", get(account_by_name))
        .route("/characters", get(characters))
        .route("/characters/:name", get(characters_by_name))
        .with_state(state);

    println!("Running on http://localhost:3000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn accounts(State(state): State<Arc<StateDB>>) -> Response {
    let database = &state.database;

    let accounts = database.accounts().await;

    match accounts {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(accounts) => Json(accounts).into_response(),
    }
}

async fn account_by_name(State(state): State<Arc<StateDB>>, Path(name): Path<String>) -> Response {
    let database = &state.database;

    let account = database.account_by_name(&name).await;

    match account {
        Err(err) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Ok(Some(account)) => Json(account).into_response(),
    }
}

async fn characters(State(state): State<Arc<StateDB>>) -> Response {
    let database = &state.database;

    let characters = database.characters().await;

    match characters {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(characters) => Json(characters).into_response(),
    }
}

async fn characters_by_name(
    State(state): State<Arc<StateDB>>,
    Path(name): Path<String>,
) -> Response {
    let database = &state.database;

    let characters = database.character_by_name(&name).await;

    match characters {
        Err(err) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Ok(Some(characters)) => Json(characters).into_response(),
    }
}
