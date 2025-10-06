use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResp {
    /// token
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub access_token: String,
}
