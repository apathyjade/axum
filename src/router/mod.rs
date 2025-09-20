
mod user;
mod common;

// 可选：提供一个组合所有子路由的函数（也可在 main.rs 中组合）
use axum::Router;

pub fn all_routes() -> Router {
    Router::new()
        .nest("/user", user::router())    // 挂载用户路由到 /users
        .merge(common::router())           // 合并公共路由（无前缀）
}