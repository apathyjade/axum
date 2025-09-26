
use axum::{
    body::Body,
    extract::Request,
    middleware::{Next},
    response::IntoResponse,
    response::Response,
    Json
};

use crate::model::api_response::ApiResponse;

#[allow(dead_code)]
pub async fn deal_error(
    request: Request<Body>,
    next: Next,
) -> Response {
    let request = request;
    let response = next.run(request).await;
    let code = response.status();
    match code.as_u16() {
        (400..=499) => {
            return Json(ApiResponse::<()>::error("服务异常，请稍后再试~~~")).into_response();
        },
        (500..=599) => {
            return Json(ApiResponse::<()>::error("系统异常，请稍后再试~~~")).into_response();
        },
        _ => {
            return response;
        },
    }
}