use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::AppErr;

#[derive(Debug, Clone, Serialize)]
pub struct ApiRes<T> {
    code: i8,
    data: Option<T>,
    msg: Option<String>,
}

impl<T> ApiRes<T> {
    pub fn new(code: i8, data: Option<T>, msg: Option<String>) -> Self {
        Self { code, data, msg }
    }
    pub fn ok(data: T) -> Self {
        Self { code: 0, data: Some(data), msg: None }
    }
    pub fn ok_with_msg(data: T, msg: String) -> Self {
        Self { code: 0, data: Some(data), msg: Some(msg) }
    }
    pub fn biz_err(msg: String) -> Self {
        Self { code: 1, data: None, msg: Some(msg) }
    }
    pub fn biz_err_with_data(msg: String, data: T) -> Self {
        Self { code: 1, data: Some(data), msg: Some(msg) }
    }
    pub fn service_err(msg: String) -> Self {
        Self { code: 2, data: None, msg: Some(msg) }
    }
    pub fn no_login(msg: String) -> Self {
        Self { code: 3, data: None, msg: Some(msg) }
    }
    pub fn no_auth(msg: String) -> Self {
        Self { code: 4, data: None, msg: Some(msg) }
    }
}

impl<T> From<diesel::result::Error> for ApiRes<T> {
    fn from(_: diesel::result::Error) -> Self {
        ApiRes::service_err("数据存储异常！".to_string())
    }
}

impl<T> From<AppErr> for ApiRes<T> {
    fn from(err: AppErr) -> Self {
        ApiRes::service_err(err.message())
    }
}

impl<T> IntoResponse for ApiRes<T> where T: Serialize {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
