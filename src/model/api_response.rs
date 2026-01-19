use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[repr(i32)]
pub enum ResCodeEnum {
    Success = 0,
    BizError = 1,
    ServiceError = 2,
    NoLogin = 3,
    NoAuth = 4,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: ResCodeEnum,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl<T> ApiResponse<T> {
    #[allow(dead_code)]
    pub fn new(code: ResCodeEnum, data: Option<T>, msg: Option<String>) -> Self {
        Self { code, data, msg }
    }
    pub fn success(data: T) -> Self {
        Self {
            code: ResCodeEnum::Success,
            data: Some(data),
            msg: None,
        }
    }
    pub fn error(msg: &str) -> Self {
        Self {
            code: ResCodeEnum::BizError,
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn error_with_code(msg: &str, code: ResCodeEnum) -> Self {
        Self {
            code,
            data: None,
            msg: Some(msg.to_string()),
        }
    }
}
