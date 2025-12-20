use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResp {
    /// Access Token
    #[schema(
        example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1NTBlODQwMC1lMjliLTQxZDQtYTcxNi00NDY2NTU0NDAwMDAiLCJleHAiOjE3Mjk2MTE2MDB9.abc123"
    )]
    pub access_token: String,

    /// Refresh Token  
    #[schema(example = "v2VjbGQgc3RhcnRpdmV0b3JrZXktZG9lcyBvZmZpbmU=")]
    pub refresh_token: String,
}
