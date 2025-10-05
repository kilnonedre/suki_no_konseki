use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum UserProfiles {
    Table,
    Id,
    RealName,
    IdCardNo,
    Gender,
    Birthday,
    Address,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    UpdatedReason,
}
