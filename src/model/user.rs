
use serde::{ Deserialize, Serialize};
use chrono::{ NaiveDateTime };
use diesel::{
    prelude::*,
    Queryable,
    QueryableByName,
    AsChangeset,
};
use crate::schema;

// 示例用户模型
// #[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize)]
#[derive(Queryable, QueryableByName, AsChangeset, Selectable)]
#[diesel(table_name = schema::user)]
pub struct ViewUser {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub real_name: Option<String>,
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