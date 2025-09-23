
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            msg: None,
        }
    }
    pub fn error(msg: &str) -> Self {
        Self {
            code: 1,
            data:None,
            msg: Some(msg.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn error_with_code(msg: &str, code: i32) -> Self {
        Self {
            code: code,
            data:None,
            msg: Some(msg.to_string()),
        }
    }
}