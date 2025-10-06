use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct LoginReq {
    /// 账号
    #[schema(example = "xxxxxxxxx")]
    pub account: String,

    /// 密码
    #[schema(example = "xxxxxxxxx")]
    pub password: String,
}
