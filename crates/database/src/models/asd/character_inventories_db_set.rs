#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CharacterInventories;

pub struct CharacterInventoriesSet;

impl CharacterInventoriesSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CharacterInventories>> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<CharacterInventories> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<CharacterInventories>> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<CharacterInventories>> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CharacterInventories> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CharacterInventories>> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CharacterInventories>> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<CharacterInventories>> {
        query_as::<_, CharacterInventories>(r#"SELECT * FROM "character_inventories" WHERE name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, character_inventories: CharacterInventories) -> Result<CharacterInventories> {
        query_as::<_, CharacterInventories>(r#"INSERT INTO "character_inventories" ("name", "value") VALUES ($1, $2) RETURNING *;"#)
            .bind(character_inventories.name)
            .bind(character_inventories.value)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, character_inventories: CharacterInventories) -> Result<CharacterInventories> {
        query_as::<_, CharacterInventories>(r#"UPDATE "character_inventories" SET "value" = $2 WHERE "name" = 1 RETURNING *;"#)
            .bind(character_inventories.name)
            .bind(character_inventories.value)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "character_inventories" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
