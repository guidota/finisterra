#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

use sqlx::{query, query_as, PgExecutor, Result};
use super::CharacterEquipment;

pub struct CharacterEquipmentSet;

impl CharacterEquipmentSet {
    pub async fn all<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Vec<CharacterEquipment>> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment""#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<CharacterEquipment> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE "name" = $1"#)
            .bind(name)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, name_list: Vec<String>) -> Result<Vec<CharacterEquipment>> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE "name" = ANY($1)"#)
            .bind(name_list)
            .fetch_all(executor)
            .await
    }

    pub async fn by_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, name: String) -> Result<Option<CharacterEquipment>> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE "name" = $1"#)
            .bind(name)
            .fetch_optional(executor)
            .await
    }

    pub async fn by_character_name<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<CharacterEquipment> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE "#)
            .fetch_one(executor)
            .await
    }

    pub async fn many_by_character_name_list<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Vec<CharacterEquipment>> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE "#)
            .fetch_all(executor)
            .await
    }

    pub async fn by_character_name_optional<'e, E: PgExecutor<'e>>(&self, executor: E, ) -> Result<Option<CharacterEquipment>> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE "#)
            .fetch_optional(executor)
            .await
    }


    pub async fn all_by_characters_name<'e, E: PgExecutor<'e>>(executor: E, characters_name: String) -> Result<Vec<CharacterEquipment>> {
        query_as::<_, CharacterEquipment>(r#"SELECT * FROM "character_equipment" WHERE name = $1"#)
            .bind(characters_name)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: PgExecutor<'e>>(&self, executor: E, character_equipment: CharacterEquipment) -> Result<CharacterEquipment> {
        query_as::<_, CharacterEquipment>(r#"INSERT INTO "character_equipment" ("name", "body", "face", "skin", "hair") VALUES ($1, $2, $3, $4, $5) RETURNING *;"#)
            .bind(character_equipment.name)
            .bind(character_equipment.body)
            .bind(character_equipment.face)
            .bind(character_equipment.skin)
            .bind(character_equipment.hair)
            .fetch_one(executor)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E, character_equipment: CharacterEquipment) -> Result<CharacterEquipment> {
        query_as::<_, CharacterEquipment>(r#"UPDATE "character_equipment" SET "body" = $2, "face" = $3, "skin" = $4, "hair" = $5 WHERE "name" = 1 RETURNING *;"#)
            .bind(character_equipment.name)
            .bind(character_equipment.body)
            .bind(character_equipment.face)
            .bind(character_equipment.skin)
            .bind(character_equipment.hair)
            .fetch_one(executor)
            .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<()> {
        query(r#"DELETE FROM "character_equipment" WHERE "name" = 1"#)
            .execute(executor)
            .await
            .map(|_| ())
    }

}
