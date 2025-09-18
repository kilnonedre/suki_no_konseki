use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[schema(description = "创建或查询客户时返回的响应对象")]
pub struct CustomerResp {
    /// 客户的唯一标识
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    /// 客户名称
    #[schema(example = "ACME Ltd.")]
    pub name: String,
}
