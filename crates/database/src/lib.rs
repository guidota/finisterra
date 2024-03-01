use std::env;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, migrate::MigrateDatabase, SqlitePool};

pub struct Database {
    pool: SqlitePool,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub mail: String,
    pub password: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub account_name: String,
}

impl Database {
    pub async fn initialize() -> Result<Self> {
        let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:argentum.db".to_string());
        if !sqlx::Sqlite::database_exists(&db_url).await? {
            sqlx::Sqlite::create_database(&db_url).await?;
        }

        let pool = SqlitePool::connect(&db_url).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
    pub async fn create_account(&self, mail: &str, password: &str) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;

        // Insert the task, then obtain the ID of this row
        let id = sqlx::query(
            "
                INSERT INTO accounts ( mail, password )
                VALUES ( $1, $2 )
            ",
        )
        .bind(mail)
        .bind(password)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn accounts(&self) -> Result<Vec<Account>> {
        let mut conn = self.pool.acquire().await?;

        let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
            .fetch_all(&mut *conn)
            .await?;

        Ok(accounts)
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

    pub async fn account_characters(&self, account_name: &str) -> Result<Vec<Character>> {
        let mut conn = self.pool.acquire().await?;
        let characters = sqlx::query_as::<_, Character>(
            "SELECT account_name, name FROM characters WHERE account_name = $1",
        )
        .bind(account_name)
        .fetch_all(&mut *conn)
        .await?;

        Ok(characters)
    }

    pub async fn characters(&self) -> Result<Vec<Character>> {
        let mut conn = self.pool.acquire().await?;

        let characters = sqlx::query_as::<_, Character>("SELECT * FROM characters")
            .fetch_all(&mut *conn)
            .await?;

        Ok(characters)
    }

    pub async fn character_by_name(&self, mail: &str) -> Result<Option<Character>> {
        let mut conn = self.pool.acquire().await?;
        let character = sqlx::query_as::<_, Character>("SELECT * FROM characters WHERE name = $1")
            .bind(mail)
            .fetch_one(&mut *conn)
            .await;

        match character {
            Ok(character) => Ok(Some(character)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }

    pub async fn insert_character(&self, account_id: i64, character: &str) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;
        let id = sqlx::query(
            "
                INSERT INTO characters ( account_id, name )
                VALUES ( $1, $2 )
            ",
        )
        .bind(account_id)
        .bind(character)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
