use sea_orm_migration::prelude::*;

use crate::{
    env::DB_SCHEMA,
    iden::users::{Users, UsersAud},
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
                        Users::Table.into_iden(),
                    ))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                            .comment("UUID 主键"),
                    )
                    .col(
                        ColumnDef::new(Users::EntSt)
                            .custom(Alias::new(&format!("{}.entity_status", *DB_SCHEMA)))
                            .not_null()
                            .default(Expr::cust(&format!(
                                "'ACTIVE'::{}.entity_status",
                                *DB_SCHEMA
                            )))
                            .comment("实体状态"),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(50)
                            .unique_key()
                            .not_null()
                            .comment("用户名"),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(100)
                            .unique_key()
                            .null()
                            .comment("邮箱地址"),
                    )
                    .col(
                        ColumnDef::new(Users::Phone)
                            .string_len(20)
                            .null()
                            .comment("手机号"),
                    )
                    .col(
                        ColumnDef::new(Users::PasswordHash)
                            .text()
                            .not_null()
                            .comment("密码哈希"),
                    )
                    .col(
                        ColumnDef::new(Users::DisplayName)
                            .string_len(100)
                            .null()
                            .comment("显示名称"),
                    )
                    .col(
                        ColumnDef::new(Users::Role)
                            .string_len(30)
                            .not_null()
                            .comment("角色 / 权限"),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedBy)
                            .uuid()
                            .null()
                            .comment("创建人 ID"),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedBy)
                            .uuid()
                            .null()
                            .comment("更新人 ID"),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedReason)
                            .text()
                            .null()
                            .comment("更新原因"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UsersAud::Table.into_iden(),
                    ))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UsersAud::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                            .comment("审计记录ID"),
                    )
                    .col(
                        ColumnDef::new(UsersAud::AudSt)
                            .custom(Alias::new(&format!("{}.audit_status", *DB_SCHEMA)))
                            .not_null()
                            .comment("审计模式：CREATE / UPDATE / DELETE"),
                    )
                    .col(
                        ColumnDef::new(UsersAud::EntId)
                            .uuid()
                            .null()
                            .comment("隶属用户 ID（外键 -> users.id）"),
                    )
                    // —— 快照字段（与 users 对齐；均可空，允许删除时保留快照）——
                    .col(
                        ColumnDef::new(UsersAud::EntSt)
                            .custom(Alias::new(&format!("{}.entity_status", *DB_SCHEMA)))
                            .null(),
                    )
                    .col(ColumnDef::new(UsersAud::Username).string_len(50).null())
                    .col(ColumnDef::new(UsersAud::Email).string_len(100).null())
                    .col(ColumnDef::new(UsersAud::Phone).string_len(20).null())
                    .col(ColumnDef::new(UsersAud::PasswordHash).text().null())
                    .col(ColumnDef::new(UsersAud::DisplayName).string_len(100).null())
                    .col(ColumnDef::new(UsersAud::Role).string_len(30).null())
                    .col(
                        ColumnDef::new(UsersAud::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UsersAud::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(ColumnDef::new(UsersAud::CreatedBy).uuid().null())
                    .col(ColumnDef::new(UsersAud::UpdatedBy).uuid().null())
                    .col(ColumnDef::new(UsersAud::UpdatedReason).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_users_aud_ent_id__users_id")
                    .from_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UsersAud::Table.into_iden(),
                    ))
                    .from_col(UsersAud::EntId)
                    .to_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        Users::Table.into_iden(),
                    ))
                    .to_col(Users::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_aud_ent_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UsersAud::Table.into_iden(),
                    ))
                    .col(UsersAud::EntId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_users_aud_ent_id__users_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UsersAud::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UsersAud::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        Users::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await
    }
}
