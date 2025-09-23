// 创建模型定义文件

use serde::{ Deserialize, Serialize};

// 示例用户模型
#[derive(sqlx::FromRow)]
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
}


#[derive(sqlx::FromRow)]
#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub name: Option<String>,
    pub email: Option<String>,
}