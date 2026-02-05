use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use utoipa::ToSchema;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
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
    /// 响应码：<br>
    /// 0 = 成功<br>
    /// 1 = 业务错误<br>
    /// 2 = 服务异常<br>
    /// 3 = 未登录<br>
    /// 4 = 无权限
    #[schema(
        value_type = i32,
        example = 0,
    )]
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


#[derive(Debug)]
pub enum AppErr {
    DatabaseError(String),
    DataExists(String),
}

impl AppErr {
    pub fn message(&self) -> String {
        match self {
            AppErr::DatabaseError(msg) => msg.to_string(),
            AppErr::DataExists(msg) => msg.to_string(),
        }
    }
}

impl From<diesel::result::Error> for AppErr {
    fn from(_: diesel::result::Error) -> Self {
        AppErr::DatabaseError("数据存储异常！".to_string())
    }
}