use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseT<T> {
    pub code: String,
    pub data: Option<T>,
    pub msg: String,
}

impl<T> ResponseT<T> {
    pub fn new(code: impl Into<String>, data: Option<T>, msg: impl Into<Option<String>>) -> Self {
        Self {
            code: code.into(),
            data,
            msg: msg.into().unwrap_or_default(),
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new("200", Some(data), Some("ok".to_string()))
    }

    pub fn err(svc: u16, err_no: u16, msg: impl Into<Option<String>>) -> Self {
        // 服务码不补零，错误码3位补零 => 40 + 001 = "40001"
        Self::new(format!("{svc:03}{err_no:03}"), None, msg)
    }
}
