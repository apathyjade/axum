use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
};

// 健康检查
async fn health_check() -> &'static str {
    "OK"
}

// 404 处理
async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "页面不存在")
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .fallback(not_found)
}