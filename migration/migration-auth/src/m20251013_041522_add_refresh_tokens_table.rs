use sea_orm_migration::prelude::*;

use crate::{
    env::DB_SCHEMA,
    iden::refresh_tokens::{RefreshTokens, RefreshTokensAud},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokens::Table.into_iden(),
                    ))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RefreshTokens::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                            .comment("UUID 主键"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::EntSt)
                            .custom(Alias::new(&format!("{}.entity_status", *DB_SCHEMA)))
                            .not_null()
                            .default(Expr::cust(&format!(
                                "'ACTIVE'::{}.entity_status",
                                *DB_SCHEMA
                            )))
                            .comment("实体状态"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::PrevId)
                            .uuid()
                            .null()
                            .comment("上一次轮换 ID"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::UserId)
                            .uuid()
                            .not_null()
                            .comment("用户 ID"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::RefreshHash)
                            .string_len(128)
                            .not_null()
                            .comment("Refresh Token Hash"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::IpAddress)
                            .string_len(45)
                            .null()
                            .comment("客户端 IP 地址"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::UserAgent)
                            .text()
                            .null()
                            .comment("客户端 User-Agent"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::Exp)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("过期时间"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::Iat)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("签发时间"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::DeviceId)
                            .string_len(128)
                            .null()
                            .comment("设备 ID"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::Version)
                            .string_len(16)
                            .not_null()
                            .comment("Token 版本"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::IsRevoked)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否撤销"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::CreatedBy)
                            .uuid()
                            .not_null()
                            .comment("创建人 ID"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::UpdatedBy)
                            .uuid()
                            .null()
                            .comment("更新人 ID"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::UpdatedReason)
                            .text()
                            .null()
                            .comment("更新理由"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokensAud::Table.into_iden(),
                    ))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RefreshTokensAud::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                            .comment("审计记录ID"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokensAud::AudSt)
                            .custom(Alias::new(&format!("{}.audit_status", *DB_SCHEMA)))
                            .not_null()
                            .comment("审计模式：CREATE / UPDATE / DELETE"),
                    )
                    .col(
                        ColumnDef::new(RefreshTokensAud::EntId)
                            .uuid()
                            .null()
                            .comment("隶属用户 ID（外键 -> users.id）"),
                    )
                    // —— 快照字段（与 users 对齐；均可空，允许删除时保留快照）——
                    .col(
                        ColumnDef::new(RefreshTokensAud::EntSt)
                            .custom(Alias::new(&format!("{}.entity_status", *DB_SCHEMA)))
                            .null(),
                    )
                    .col(ColumnDef::new(RefreshTokensAud::PrevId).uuid().null())
                    .col(ColumnDef::new(RefreshTokensAud::UserId).uuid().null())
                    .col(
                        ColumnDef::new(RefreshTokensAud::RefreshHash)
                            .string_len(128)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokensAud::IpAddress)
                            .string_len(45)
                            .null(),
                    )
                    .col(ColumnDef::new(RefreshTokensAud::UserAgent).text().null())
                    .col(
                        ColumnDef::new(RefreshTokensAud::Exp)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokensAud::Iat)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(ColumnDef::new(RefreshTokensAud::IsRevoked).boolean().null())
                    .col(
                        ColumnDef::new(RefreshTokensAud::DeviceId)
                            .string_len(128)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokensAud::Version)
                            .string_len(16)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokensAud::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(ColumnDef::new(RefreshTokensAud::CreatedBy).uuid().null())
                    .col(
                        ColumnDef::new(RefreshTokens::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(ColumnDef::new(RefreshTokens::UpdatedBy).uuid().null())
                    .col(ColumnDef::new(RefreshTokens::UpdatedReason).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_refresh_tokens_aud_ent_id__refresh_tokens_id")
                    .from_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokensAud::Table.into_iden(),
                    ))
                    .from_col(RefreshTokensAud::EntId)
                    .to_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokens::Table.into_iden(),
                    ))
                    .to_col(RefreshTokens::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_refresh_tokens_prev_id__refresh_tokens_id")
                    .from_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokens::Table.into_iden(),
                    ))
                    .from_col(RefreshTokens::PrevId)
                    .to_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokens::Table.into_iden(),
                    ))
                    .to_col(RefreshTokens::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_refresh_tokens_aud_ent_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokensAud::Table.into_iden(),
                    ))
                    .col(RefreshTokensAud::EntId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_refresh_tokens_aud_ent_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokensAud::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_refresh_tokens_aud_ent_id__refresh_tokens_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokensAud::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_refresh_tokens_prev_id__refresh_tokens_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokens::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokensAud::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        RefreshTokens::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
