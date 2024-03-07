#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CharacterStatistics;

pub struct CharacterStatisticsSet;

impl CharacterStatisticsSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CharacterStatistics>> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<CharacterStatistics> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<CharacterStatistics>> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<CharacterStatistics>> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CharacterStatistics> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CharacterStatistics>> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CharacterStatistics>> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<CharacterStatistics>> {
        query_as::<_, CharacterStatistics>(r#"SELECT * FROM "character_statistics" WHERE name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, character_statistics: CharacterStatistics) -> Result<CharacterStatistics> {
        query_as::<_, CharacterStatistics>(r#"INSERT INTO "character_statistics" ("name", "health", "mana", "stamina", "max_health", "max_mana", "max_stamina") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#)
            .bind(character_statistics.name)
            .bind(character_statistics.health)
            .bind(character_statistics.mana)
            .bind(character_statistics.stamina)
            .bind(character_statistics.max_health)
            .bind(character_statistics.max_mana)
            .bind(character_statistics.max_stamina)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, character_statistics: CharacterStatistics) -> Result<CharacterStatistics> {
        query_as::<_, CharacterStatistics>(r#"UPDATE "character_statistics" SET "health" = $2, "mana" = $3, "stamina" = $4, "max_health" = $5, "max_mana" = $6, "max_stamina" = $7 WHERE "name" = 1 RETURNING *;"#)
            .bind(character_statistics.name)
            .bind(character_statistics.health)
            .bind(character_statistics.mana)
            .bind(character_statistics.stamina)
            .bind(character_statistics.max_health)
            .bind(character_statistics.max_mana)
            .bind(character_statistics.max_stamina)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "character_statistics" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
