
use serde::{ Deserialize, Serialize};
use chrono::{ NaiveDateTime };
use diesel::{
    prelude::*,
    AsChangeset,
};

use crate::schema;

// 示例用户模型
// #[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize)]
#[derive(Queryable, AsChangeset)]
#[diesel(table_name = schema::user)]
pub struct ViewUser<'a> {
    pub id: i64,
    pub username: &'a str,
    pub password: String,
    pub email: &'a str,
    pub phone: &'a str,
    pub real_name: &'a str,
    pub status: i32,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
}


// #[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize)]
#[derive(Insertable)]
#[diesel(table_name = schema::user)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: String,
    pub email: &'a str,
    pub phone: &'a str,
    pub real_name: &'a str,
    pub status: i32,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
}