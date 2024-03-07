#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct CharacterAttributes {
  pub name: String,
  pub strength: i32,
  pub agility: i32,
  pub intelligence: i32,
  pub charisma: i32,
  pub constitution: i32,
}
