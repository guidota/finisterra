#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::AccountCharacters;

pub struct AccountCharactersSet;

impl AccountCharactersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<AccountCharacters> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE "id" = $1"#)
            .bind(id)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_id_list<'e, E: PgExecutor<'e>>(&self, executor: E, id_list: Vec<i32>) -> Result<Vec<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE "id" = ANY($1)"#)
            .bind(id_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_id_optional<'e, E: PgExecutor<'e>>(&self, executor: E, id: i32) -> Result<Option<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, character_name: String) -> Result<AccountCharacters> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE "character_name" = $1"#)
            .bind(character_name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, character_name_list: Vec<String>) -> Result<Vec<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE "character_name" = ANY($1)"#)
            .bind(character_name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, character_name: String) -> Result<Option<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE "character_name" = $1"#)
            .bind(character_name)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_accounts_name<'e, E: PgExecutor<'e>>(executor: E, accounts_name: String) -> Result<Vec<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE account_name = $1"#)
            .bind(accounts_name)
            .fetch_all(executor)
            .await
    }
    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<AccountCharacters>> {
        query_as::<_, AccountCharacters>(r#"SELECT * FROM "account_characters" WHERE character_name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, account_characters: AccountCharacters) -> Result<AccountCharacters> {
        query_as::<_, AccountCharacters>(r#"INSERT INTO "account_characters" ("id", "account_name", "character_name") VALUES ($1, $2, $3) RETURNING *;"#)
            .bind(account_characters.id)
            .bind(account_characters.account_name)
            .bind(account_characters.character_name)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, account_characters: AccountCharacters) -> Result<AccountCharacters> {
        query_as::<_, AccountCharacters>(r#"UPDATE "account_characters" SET "account_name" = $2, "character_name" = $3 WHERE "id" = 1 RETURNING *;"#)
            .bind(account_characters.id)
            .bind(account_characters.account_name)
            .bind(account_characters.character_name)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "account_characters" WHERE "id" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
