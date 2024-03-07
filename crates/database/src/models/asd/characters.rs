#![allow(dead_code)]
// Generated with sql-gen
// https://github.com/jayy-lmao/sql-gen

#[derive(sqlx::FromRow, Debug)]
pub struct Characters {
  pub name: String,
  pub description: Option<String>,
  pub level: Option<i32>,
  pub exp: Option<i32>,
  pub class_id: i32,
  pub race_id: i32,
  pub gender_id: i32,
  pub created_at: chrono::NaiveDateTime,
}
