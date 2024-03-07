#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Accounts;

pub struct AccountsSet;

impl AccountsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Accounts>> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Accounts> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<Accounts>> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<Accounts>> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Accounts> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Accounts>> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Accounts>> {
        query_as::<_, Accounts>(r#"SELECT * FROM "accounts" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, accounts: Accounts) -> Result<Accounts> {
        query_as::<_, Accounts>(r#"INSERT INTO "accounts" ("name", "email", "password", "pin", "created_at") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(accounts.name)
            .bind(accounts.email)
            .bind(accounts.password)
            .bind(accounts.pin)
            .bind(accounts.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, accounts: Accounts) -> Result<Accounts> {
        query_as::<_, Accounts>(r#"UPDATE "accounts" SET "email" = $2, "password" = $3, "pin" = $4, "created_at" = $5 WHERE "name" = 1 RETURNING *;"#)
            .bind(accounts.name)
            .bind(accounts.email)
            .bind(accounts.password)
            .bind(accounts.pin)
            .bind(accounts.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "accounts" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
