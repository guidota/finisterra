#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CharacterAttributes;

pub struct CharacterAttributesSet;

impl CharacterAttributesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CharacterAttributes>> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<CharacterAttributes> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<CharacterAttributes>> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<CharacterAttributes>> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CharacterAttributes> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CharacterAttributes>> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CharacterAttributes>> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<CharacterAttributes>> {
        query_as::<_, CharacterAttributes>(r#"SELECT * FROM "character_attributes" WHERE name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, character_attributes: CharacterAttributes) -> Result<CharacterAttributes> {
        query_as::<_, CharacterAttributes>(r#"INSERT INTO "character_attributes" ("name", "strength", "agility", "intelligence", "charisma", "constitution") VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#)
            .bind(character_attributes.name)
            .bind(character_attributes.strength)
            .bind(character_attributes.agility)
            .bind(character_attributes.intelligence)
            .bind(character_attributes.charisma)
            .bind(character_attributes.constitution)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, character_attributes: CharacterAttributes) -> Result<CharacterAttributes> {
        query_as::<_, CharacterAttributes>(r#"UPDATE "character_attributes" SET "strength" = $2, "agility" = $3, "intelligence" = $4, "charisma" = $5, "constitution" = $6 WHERE "name" = 1 RETURNING *;"#)
            .bind(character_attributes.name)
            .bind(character_attributes.strength)
            .bind(character_attributes.agility)
            .bind(character_attributes.intelligence)
            .bind(character_attributes.charisma)
            .bind(character_attributes.constitution)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "character_attributes" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
