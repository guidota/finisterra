#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct AccountCharacters {
  pub id: i32,
  pub account_name: String,
  pub character_name: String,
}
