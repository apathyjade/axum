use crate::{model::pager::Pager, schema};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// 示例用户模型
#[derive(Deserialize, Serialize, ToSchema, Queryable, QueryableByName, AsChangeset, Selectable)]
#[diesel(table_name = schema::users)]
#[schema(
    example = json!({
        "id": 1,
        "username": "testuser",
        "password": "password123",
        "email": "test@example.com",
        "phone": "1234567890",
        "real_name": "Test User",
        "status": 0,
        "created_time": "2023-07-01 12:00:00",
        "updated_time": "2023-07-01 12:00:00"
    })
)]
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
#[derive(Deserialize, Serialize, ToSchema, Insertable)]
#[diesel(table_name = schema::users)]
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
#[derive(Deserialize, Serialize, ToSchema)]
pub struct ListQueryParams {
    pub pager: Option<Pager>,
    pub search: Option<String>,
}
