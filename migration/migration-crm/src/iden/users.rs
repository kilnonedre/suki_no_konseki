use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    EntSt,
    Username,
    Email,
    Phone,
    PasswordHash,
    DisplayName,
    Role,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    UpdatedReason,
}

#[derive(DeriveIden)]
pub enum UsersAud {
    Table,
    Id,
    AudSt,
    EntId,
    EntSt,
    Username,
    Email,
    Phone,
    PasswordHash,
    DisplayName,
    Role,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    UpdatedReason,
}
