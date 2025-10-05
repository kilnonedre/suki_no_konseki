use common_type::EntityStatus;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct UserResp {
    /// 客户的唯一标识
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    /// 实体状态
    #[schema(example = "ACTIVE")]
    pub ent_st: EntityStatus,

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

    /// 创建时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub created_at: DateTimeWithTimeZone,

    /// 更新时间
    #[schema(value_type = String, format = DateTime, example = "2025-09-25T08:00:00Z")]
    pub updated_at: DateTimeWithTimeZone,

    /// 创建人
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub created_by: Option<Uuid>,

    /// 更新人
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub updated_by: Option<Uuid>,

    /// 更新原因
    #[schema(example = "xxxxxxxxx")]
    pub updated_reason: Option<String>,
}
