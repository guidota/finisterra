#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct CharacterStatistics {
  pub name: String,
  pub health: i32,
  pub mana: i32,
  pub stamina: i32,
  pub max_health: i32,
  pub max_mana: i32,
  pub max_stamina: i32,
}
