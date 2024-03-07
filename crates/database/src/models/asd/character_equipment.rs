#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct CharacterEquipment {
  pub name: String,
  pub body: i32,
  pub face: i32,
  pub skin: i32,
  pub hair: i32,
}
