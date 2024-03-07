#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::Characters;

pub struct CharactersSet;

impl CharactersSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<Characters>> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Characters> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<Characters>> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<Characters>> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Characters> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<Characters>> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<Characters>> {
        query_as::<_, Characters>(r#"SELECT * FROM "characters" WHERE "#)
            .fetch_optional(executor)
            .await
    }



    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, characters: Characters) -> Result<Characters> {
        query_as::<_, Characters>(r#"INSERT INTO "characters" ("name", "description", "level", "exp", "class_id", "race_id", "gender_id", "created_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;"#)
            .bind(characters.name)
            .bind(characters.description)
            .bind(characters.level)
            .bind(characters.exp)
            .bind(characters.class_id)
            .bind(characters.race_id)
            .bind(characters.gender_id)
            .bind(characters.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, characters: Characters) -> Result<Characters> {
        query_as::<_, Characters>(r#"UPDATE "characters" SET "description" = $2, "level" = $3, "exp" = $4, "class_id" = $5, "race_id" = $6, "gender_id" = $7, "created_at" = $8 WHERE "name" = 1 RETURNING *;"#)
            .bind(characters.name)
            .bind(characters.description)
            .bind(characters.level)
            .bind(characters.exp)
            .bind(characters.class_id)
            .bind(characters.race_id)
            .bind(characters.gender_id)
            .bind(characters.created_at)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "characters" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
