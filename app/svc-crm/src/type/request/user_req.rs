use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct UserBase {
    /// 用户名
    #[schema(example = "xxx")]
    pub username: String,

    /// 邮箱
    #[schema(example = "xxxxx@xx.com")]
    pub email: Option<String>,

    /// 手机号
    #[schema(example = "13111111111")]
    pub phone: Option<String>,

    /// 显示名称
    #[schema(example = "xxxxxxxxx")]
    pub display_name: Option<String>,

    /// 角色
    #[schema(example = "xxxxxxxxx")]
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct CreateUserReq {
    #[serde(flatten)]
    pub base: UserBase,

    /// 密码
    #[schema(example = "xxxxxxxxx")]
    pub password: String,
}

/// 全量更新（PUT）：公共字段（不含密码）
#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct UpdateUserReq {
    #[serde(flatten)]
    pub base: UserBase,
}
