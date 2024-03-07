#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CharacterSpellbooks;

pub struct CharacterSpellbooksSet;

impl CharacterSpellbooksSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CharacterSpellbooks>> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<CharacterSpellbooks> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<CharacterSpellbooks>> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<CharacterSpellbooks>> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CharacterSpellbooks> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CharacterSpellbooks>> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CharacterSpellbooks>> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<CharacterSpellbooks>> {
        query_as::<_, CharacterSpellbooks>(r#"SELECT * FROM "character_spellbooks" WHERE name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, character_spellbooks: CharacterSpellbooks) -> Result<CharacterSpellbooks> {
        query_as::<_, CharacterSpellbooks>(r#"INSERT INTO "character_spellbooks" ("name", "value") VALUES ($1, $2) RETURNING *;"#)
            .bind(character_spellbooks.name)
            .bind(character_spellbooks.value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, character_spellbooks: CharacterSpellbooks) -> Result<CharacterSpellbooks> {
        query_as::<_, CharacterSpellbooks>(r#"UPDATE "character_spellbooks" SET "value" = $2 WHERE "name" = 1 RETURNING *;"#)
            .bind(character_spellbooks.name)
            .bind(character_spellbooks.value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "character_spellbooks" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
