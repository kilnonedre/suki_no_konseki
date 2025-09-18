// common-entity/src/status.rs
use sea_orm::entity::prelude::*;

/// 通用的实体状态
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "entity_status")]
pub enum EntityStatus {
    #[sea_orm(string_value = "ACTIVE")]
    Active,
    #[sea_orm(string_value = "INACTIVE")]
    Inactive,
    #[sea_orm(string_value = "DELETED")]
    Deleted,
}
