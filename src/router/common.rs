use axum::{Router, http::{HeaderValue, StatusCode, header}, response::IntoResponse, routing::get};

use hyper::{Request, Response};
use tower_http::{
    ServiceExt, services::{ServeDir, ServeFile}
};

use crate::{AppStateArc};

use utoipa_swagger_ui::SwaggerUi;

include!("../api_doc.rs");

// 健康检查
async fn health_check() -> &'static str {
    "OK"
}

// 404 处理
async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "页面不存在")
}

pub fn router() -> Router<AppStateArc> {
    let serve_dir = ServeDir::new("web/apps/shell/dist")
        .append_index_html_on_directories(true) // 访问目录时自动加 /index.html
        .fallback(ServeFile::new("web/apps/shell/dist/index.html"));
    let spe = ApiDoc::openapi();

    let tiles_dir = ServeDir::new("public/tiles")
        .append_index_html_on_directories(false)
        .precompressed_gzip()
        .append_response_header(header::CONTENT_TYPE, HeaderValue::from_static("application/x-protobuf"))
        .append_response_header(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));

    Router::new()
        .nest_service("/web", serve_dir)
        .nest_service("/tiles", tiles_dir)
        // .nest_service("/forward", get(forward_request))
        .route("/health", get(health_check))
        .merge(SwaggerUi::new("/swagger-ui")
               .url("/api-docs/openapi.json", spe))
        .fallback(not_found)
}
