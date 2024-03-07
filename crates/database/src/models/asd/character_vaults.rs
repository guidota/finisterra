#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct CharacterVaults {
  pub name: String,
  pub value: Vec<u8>,
}
