#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Accounts {
  pub name: String,
  pub email: String,
  pub password: String,
  pub pin: i32,
  pub created_at: chrono::NaiveDateTime,
}
