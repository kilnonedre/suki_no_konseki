use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Default, Debug, Deserialize, ToSchema, Clone)]
pub struct RefreshTokenReq {
    /// 上一次轮换的记录 ID（自引用）
    #[schema(example = "a9b9c8d7-e6f5-4433-b222-111111111111", nullable = true)]
    pub prev_id: Option<Uuid>,

    /// 用户 ID
    #[schema(example = "d1f1e1b1-1111-2222-3333-444455556666")]
    pub user_id: Uuid,

    /// Refresh Token 的哈希（仅存哈希，不存明文）
    #[schema(example = "f3b1d9d4e8...64hex", min_length = 44, max_length = 128)]
    pub refresh_hash: String,

    /// 客户端 IP
    #[schema(example = "192.168.1.10", nullable = true)]
    pub ip_address: Option<String>,

    /// User-Agent
    #[schema(example = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)", nullable = true)]
    pub user_agent: Option<String>,

    /// 到期时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub exp: DateTimeWithTimeZone,

    /// 签发时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub iat: DateTimeWithTimeZone,

    /// 是否已撤销
    #[schema(example = false)]
    pub is_revoked: bool,

    /// 创建人
    #[schema(example = "9f8c7b6a-5d4e-3f2a-1b0c-9e8d7c6b5a4f")]
    pub created_by: Uuid,

    /// 设备 ID
    #[schema(example = "device_12345", nullable = true)]
    pub device_id: Option<String>,

    /// Token 版本
    #[schema(example = "v1")]
    pub version: String,
}

#[derive(Default, Debug, Deserialize, ToSchema, Clone)]
pub struct RefreshTokenPatchReq {
    /// 上一次轮换的记录 ID（自引用）
    #[schema(example = "a9b9c8d7-e6f5-4433-b222-111111111111", nullable = true)]
    pub prev_id: Option<Uuid>,

    /// 是否已撤销
    #[schema(example = false)]
    pub is_revoked: Option<bool>,
}
