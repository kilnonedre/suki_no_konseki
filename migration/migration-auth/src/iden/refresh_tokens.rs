use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum RefreshTokens {
    Table,
    Id,
    EntSt,
    PrevId,
    UserId,
    RefreshHash,
    IpAddress,
    UserAgent,
    Exp,
    Iat,
    DeviceId,
    Version,
    IsRevoked,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    UpdatedReason,
}

#[derive(DeriveIden)]
pub enum RefreshTokensAud {
    Table,
    Id,
    AudSt,
    EntId,
    EntSt,
    PrevId,
    UserId,
    RefreshHash,
    IpAddress,
    UserAgent,
    Exp,
    Iat,
    DeviceId,
    Version,
    IsRevoked,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    UpdatedReason,
}
