use std::sync::Arc;

use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{Request, StatusCode},
    Json,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router, routing::{get, put},
};
use serde::{Deserialize, Serialize};

use database::{Database, MarketRepository};

struct StateDB {
    database: Database,
}

#[derive(Deserialize)]
struct Sell {
    price: i64,
    for_sale: bool,
}

impl Default for Sell {
    fn default() -> Self {
        Self {
            price: 0,
            for_sale: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Marketplace {
    operation: String,
    character_involved: String,
    price: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let database = Database::initialize().await?;

    let state = Arc::new(StateDB { database });

    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/accounts", get(accounts))
        .route("/accounts/:name", get(account_by_name))
        .route("/accounts/:name/characters", get(characters_by_account))
        .route("/characters", get(characters))
        .route("/characters/:name", get(characters_by_name))
        .route("/characters/for_sale", get(characters_for_sale))
        .route("/accounts/:account_name/characters/:name/sell", put(character_sell))
        .route("/accounts/:account_name/characters/:name", put(marketplace_operation))
        .with_state(state)
        .layer(middleware::from_fn(logging_middleware));

    println!("Running on http://localhost:3000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}

async fn accounts(State(state): State<Arc<StateDB>>) -> Response {
    let accounts = &state.database.accounts().await;

    match accounts {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(accounts) => Json(accounts).into_response(),
    }
}

async fn account_by_name(State(state): State<Arc<StateDB>>, Path(name): Path<String>) -> Response {
    let account = &state.database.account_by_name(&name).await;

    match account {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Ok(Some(account)) => Json(account).into_response(),
    }
}

async fn characters_by_account(
    State(state): State<Arc<StateDB>>,
    Path(name): Path<String>,
) -> Response {
    let characters = &state.database.account_characters(&name).await;

    match characters {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Ok(characters) => Json(characters).into_response(),
    }
}

async fn characters(State(state): State<Arc<StateDB>>) -> Response {
    let characters = &state.database.characters().await;

    match characters {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(characters) => Json(characters).into_response(),
    }
}

async fn characters_by_name(
    State(state): State<Arc<StateDB>>,
    Path(name): Path<String>,
) -> Response {
    let characters = &state.database.character_by_name(&name).await;

    match characters {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Ok(Some(characters)) => Json(characters).into_response(),
    }
}

async fn character_sell(
    State(state): State<Arc<StateDB>>,
    Path(account_name): Path<String>,
    Path(character_name): Path<String>,
    sell: Option<Query<Sell>>,
) -> Response {
    let Query(sell) = sell.unwrap_or_default();

    let updated = &state.database
        .character_sell(&account_name, &character_name, sell.price, sell.for_sale)
        .await;

    match updated {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(updated) => Json(updated).into_response(),
    }
}

async fn characters_for_sale(State(state): State<Arc<StateDB>>) -> Response {
    let characters = &state.database.characters_for_sale().await;

    match characters {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(characters) => Json(characters).into_response(),
    }
}

async fn marketplace_operation(
    State(state): State<Arc<StateDB>>,
    Path((account_name, character_name)): Path<(String, String)>,
    Json(marketplace): Json<Marketplace>,
) -> Response {
    match marketplace.operation.as_str() {
        "sell" => sell_character(state, account_name, character_name),
        "buy" => Json("buy").into_response(),
        _ => { StatusCode::BAD_REQUEST.into_response() }
    }
}

async fn sell_character(
    State(state): Arc<StateDB>,
    account_name: String,
    character_name: String,
    marketplace: Marketplace,
) -> bool {
    let updated = &state.database
        .character_sell(&account_name, &character_name, marketplace.price, marketplace.character_involved)
        .await;

    match updated {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(updated) => Json(updated).into_response(),
    }
}