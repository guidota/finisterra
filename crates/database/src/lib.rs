use std::env;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, migrate::MigrateDatabase, SqlitePool};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub mail: String,
    pub password: String,
    pub balance: i64,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub account_name: String,
    pub race: String,
    pub price: i64,
    pub is_for_sale: bool,
}

#[derive(Debug)]
pub struct Account2 {
    pub name: String,
    pub mail: String,
    pub password: String,
    pub balance: i64,
    pub characters: Vec<Character>,
}

#[async_trait]
pub trait MarketRepository {
    // Get all characters
    async fn accounts(&self) -> Result<Vec<Account>>;

    // Get Account by account name
    async fn account_by_name(&self, account_name: &str) -> Result<Option<Account>>;

    async fn account_by_name_with_characters(&self, account_name: &str) -> Result<Option<Account2>>;
    // Get characters from account by account name
    async fn account_characters(&self, account_name: &str) -> Result<Option<Vec<Character>>>;

    // Get all characters
    async fn characters(&self) -> Result<Vec<Character>>;

    // Get character by character name
    async fn character_by_name(&self, mail: &str) -> Result<Option<Character>>;

    // Get all characters for sale
    async fn characters_for_sale(&self) -> Result<Vec<Character>>;

    // Put character on sale or remove from sale
    async fn character_sell(&self, character_name: &str, price: i64) -> Result<bool>;

    //Buy PJ
    async fn character_buy(&self, account_name: &str, character_name: &str, price: i64) -> Result<bool>;
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn initialize() -> Result<Self> {
        let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:argentum".to_string());
        if !sqlx::Sqlite::database_exists(&db_url).await? {
            sqlx::Sqlite::create_database(&db_url).await?;
            println!("Database created")
        }

        println!("{}", db_url);
        let pool = SqlitePool::connect(&db_url).await?;
        // sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
    pub async fn create_account(&self, name: &str, mail: &str, password: &str) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;

        // Insert the task, then obtain the ID of this row
        let id = sqlx::query(
            "
                INSERT INTO accounts (name, mail, password)
                VALUES ( $1, $2, $3 )
            ",
        )
            .bind(name)
        .bind(mail)
        .bind(password)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn account_by_name(&self, mail: &str) -> Result<Option<Account>> {
        let mut conn = self.pool.acquire().await?;
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE name = $1")
            .bind(mail)
            .fetch_one(&mut *conn)
            .await;

        match account {
            Ok(account) => Ok(Some(account)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    pub async fn account_characters(&self, account_name: &str) -> Result<Option<Vec<Character>>> {
        let mut conn = self.pool.acquire().await?;
        let characters = sqlx::query_as::<_, Character>(
            "SELECT name, account_name, race, price, is_for_sale FROM characters WHERE account_name = $1",
        )
        .bind(account_name)
        .fetch_all(&mut *conn)
            .await;

        match characters {
            Ok(characters) => Ok(Some(characters)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    pub async fn insert_character(&self, account_name: &str, character: &str) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;
        let id = sqlx::query(
            "
                INSERT INTO characters ( name, account_name, race, price, is_for_sale )
                VALUES ( $1, $2, $3, $4, $5 )
            ",
        )
        .bind(character)
            .bind(account_name)
            .bind("gnomo")
            .bind(10)
            .bind(false)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}

#[async_trait]
impl MarketRepository for Database {
    async fn accounts(&self) -> Result<Vec<Account>> {
        let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
            .fetch_all(&self.pool)
            .await?;

        Ok(accounts)
    }

    async fn account_by_name(&self, mail: &str) -> Result<Option<Account>> {
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE name = $1")
            .bind(mail)
            .fetch_one(&self.pool)
            .await;

        match account {
            Ok(account) => Ok(Some(account)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }


    async fn account_by_name_with_characters(&self, name: &str) -> Result<Option<Account2>> {
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await;

        match account {
            Ok(account) => {
                let mut account_with_chars = Account2 {
                    name: account.name.to_string(),
                    mail: account.mail.to_string(),
                    password: account.password.to_string(),
                    balance: account.balance,
                    characters: Vec::new(),
                };

                let characters = sqlx::query_as::<_, Character>(
                    "SELECT * FROM characters WHERE account_name = $1",
                )
                    .bind(account.name)
                    .fetch_all(&self.pool)
                    .await;

                for character in characters.unwrap() {
                    account_with_chars.characters.push(character);
                }
                // match characters {
                //     Ok(characters) => {
                //         for character in characters {
                //             account_with_chars.characters.push(character);
                //         }
                //     },
                //     Err(e) => Err(anyhow::anyhow!("{}", e)),
                // };

                Ok(Some(account_with_chars))
            },
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    async fn account_characters(&self, account_name: &str) -> Result<Option<Vec<Character>>> {
        let characters = sqlx::query_as::<_, Character>(
            "SELECT name, account_name, race, price, is_for_sale FROM characters WHERE account_name = $1",
        )
            .bind(account_name)
            .fetch_all(&self.pool)
            .await;

        match characters {
            Ok(characters) => Ok(Some(characters)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    async fn characters(&self) -> Result<Vec<Character>> {
        let characters = sqlx::query_as::<_, Character>("SELECT * FROM characters")
            .fetch_all(&self.pool)
            .await?;

        Ok(characters)
    }

    async fn character_by_name(&self, mail: &str) -> Result<Option<Character>> {
        let character = sqlx::query_as::<_, Character>("SELECT * FROM characters WHERE name = $1")
            .bind(mail)
            .fetch_one(&self.pool)
            .await;

        match character {
            Ok(character) => Ok(Some(character)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    async fn characters_for_sale(&self) -> Result<Vec<Character>> {
        let characters =
            sqlx::query_as::<_, Character>("SELECT * FROM characters WHERE is_for_sale = true")
                .fetch_all(&self.pool)
                .await?;

        Ok(characters)
    }

    async fn character_sell(&self, character_involved: &str, price: i64) -> Result<bool> {
        let updated =
            sqlx::query("UPDATE characters SET price = $1, is_for_sale = true WHERE name = $2")
                .bind(price)
                .bind(character_involved)
                .execute(&self.pool)
                .await
                .is_ok();

        Ok(updated)
    }

    async fn character_buy(&self, account_name: &str, character_involved: &str, balance: i64) -> Result<bool> {
        //Ver como se arma una TX para eto

        let mut updated =
            sqlx::query("UPDATE characters SET account_name = $1, is_for_sale = false WHERE name = $2")
                .bind(account_name)
                .bind(character_involved)
                .execute(&self.pool)
                .await
                .is_ok();

        updated =
            sqlx::query("UPDATE account SET balance = $1 WHERE name = $2")
                .bind(balance)
                .bind(account_name)
                .execute(&self.pool)
                .await
                .is_ok();

        Ok(updated)
    }
}