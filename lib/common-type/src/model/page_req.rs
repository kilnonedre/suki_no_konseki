use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageReq {
    pub page: u64,
    pub size: u64,
}
