#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CharacterVaults;

pub struct CharacterVaultsSet;

impl CharacterVaultsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CharacterVaults>> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<CharacterVaults> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<CharacterVaults>> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<CharacterVaults>> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CharacterVaults> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CharacterVaults>> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CharacterVaults>> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<CharacterVaults>> {
        query_as::<_, CharacterVaults>(r#"SELECT * FROM "character_vaults" WHERE name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, character_vaults: CharacterVaults) -> Result<CharacterVaults> {
        query_as::<_, CharacterVaults>(r#"INSERT INTO "character_vaults" ("name", "value") VALUES ($1, $2) RETURNING *;"#)
            .bind(character_vaults.name)
            .bind(character_vaults.value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, character_vaults: CharacterVaults) -> Result<CharacterVaults> {
        query_as::<_, CharacterVaults>(r#"UPDATE "character_vaults" SET "value" = $2 WHERE "name" = 1 RETURNING *;"#)
            .bind(character_vaults.name)
            .bind(character_vaults.value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "character_vaults" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
