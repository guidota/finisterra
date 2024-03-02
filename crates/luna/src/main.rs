use std::sync::Arc;

use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    Json,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router, routing::{get, put},
};
use serde::{Deserialize, Serialize};

use database::{Character, Database, MarketRepository};

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
        .route("/accounts/:name", put(marketplace_operation))
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

// async fn character_sell(
//     State(state): State<Arc<StateDB>>,
//     Path(account_name): Path<String>,
//     Path(character_name): Path<String>,
//     sell: Option<Query<Sell>>,
// ) -> Response {
//     let Query(sell) = sell.unwrap_or_default();
//
//     let updated = &state.database
//         .character_sell(&account_name, &character_name, sell.price, sell.for_sale)
//         .await;
//
//     match updated {
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//         Ok(updated) => Json(updated).into_response(),
//     }
// }

async fn characters_for_sale(State(state): State<Arc<StateDB>>) -> Response {
    let characters = &state.database.characters_for_sale().await;

    match characters {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(characters) => Json(characters).into_response(),
    }
}

async fn marketplace_operation(
    State(state): State<Arc<StateDB>>,
    Path(account_name): Path<String>,
    Json(marketplace): Json<Marketplace>,
) -> Response {
    match marketplace.operation.as_str() {
        "sell" => return sell_character(&state, &account_name, &marketplace).await,
        "buy" => return buy_character(&state, &account_name, &marketplace).await,
        _ => { StatusCode::BAD_REQUEST.into_response() }
    }
}

async fn sell_character(state: &Arc<StateDB>, account_name: &String, marketplace: &Marketplace) -> Response {
    let account_characters = &state.database.account_characters(&account_name).await;
    if let Ok(Some(characters)) = account_characters {
        let mut char_belongs_to = false;
        for character in characters {
            println!("character.name == marketplace.character_involved {} - {}", character.name, marketplace.character_involved);
            if character.name == marketplace.character_involved {
                char_belongs_to = true;
                break;
            }
        }
        println!("char_belongs_to = {}", char_belongs_to);
        if !char_belongs_to {
            return Json("it's is not yours char mf").into_response()
        }
    }

    let character_to_buy = &state.database.character_by_name(&marketplace.character_involved).await;
    if let Ok(Some(character)) = character_to_buy {
        println!("character_to_buy = {:?}", character);
        if character.is_for_sale {
            return Json("already for sale bitch").into_response()
        }
    } else {
        return Json("character doesn't exist").into_response()
    }

    let updated = &state.database
        .character_sell(&marketplace.character_involved, marketplace.price)
        .await;

    println!("updated = {:?}", updated);
    match updated {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(updated) => Json(updated).into_response(),
    }
}

async fn buy_character(state: &Arc<StateDB>, account_name: &String, marketplace: &Marketplace) -> Response {
    let character_to_buy = &state.database.character_by_name(&marketplace.character_involved).await;
    let pj: &Character;
    if let Ok(Some(character)) = character_to_buy {
        println!("character_to_buy = {:?}", character);
        if !character.is_for_sale {
            return Json("isn't for sale bitch").into_response()
        }
        if account_name == &character.account_name {
            return Json("this character its already yours").into_response()
        }
        pj = character;
    } else {
        return Json("character doesn't exist").into_response()
    }

    let mut new_balance_account: i64 = 0;
    let account_with_characters = &state.database.account_by_name_with_characters(&account_name).await;
    if let Ok(Some(account)) = account_with_characters {
        println!("account = {:?}", account);
        println!("account balance {} - char price {} ", account.balance, pj.price);
        if account.balance < pj.price {
            return Json("no te alcanza la guita mostro, ponete a matar aranitas").into_response()
        }
        new_balance_account = account.balance - pj.price
    }

    let updated = &state.database
        .character_buy(&account_name, &marketplace.character_involved, new_balance_account)
        .await;

    println!("updated = {:?}", updated);
    match updated {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(updated) => Json(updated).into_response(),
    }
}
