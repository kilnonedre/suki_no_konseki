use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "audit_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditStatus {
    #[sea_orm(string_value = "CREATE")]
    Create,
    #[sea_orm(string_value = "UPDATE")]
    Update,
    #[sea_orm(string_value = "DELETE")]
    Delete,
}
