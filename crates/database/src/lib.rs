use std::env;

use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, FromRow, SqlitePool};

pub struct Database {
    pool: SqlitePool,
}

#[derive(Debug, FromRow)]
pub struct Account {
    pub id: i64,
    pub mail: String,
    pub password: String,
}

#[derive(Debug, FromRow)]
pub struct AccountCharacter {
    pub account_id: i64,
    pub name: String,
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

    pub async fn account(&self, mail: &str) -> Result<Account> {
        let mut conn = self.pool.acquire().await?;
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts as a WHERE a.mail = $1")
            .bind(mail)
            .fetch_one(&mut *conn)
            .await?;

        Ok(account)
    }

    pub async fn account_characters(&self, account_id: i64) -> Result<Vec<AccountCharacter>> {
        let mut conn = self.pool.acquire().await?;
        let characters = sqlx::query_as::<_, AccountCharacter>(
            "SELECT account_id, name FROM account_characters as a WHERE a.account_id = $1",
        )
        .bind(account_id)
        .fetch_all(&mut *conn)
        .await?;

        Ok(characters)
    }

    pub async fn insert_character(&self, account_id: i64, character: &str) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;
        let id = sqlx::query(
            "
                INSERT INTO account_characters ( account_id, name )
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
