use std::env;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = env::var("DB_SCHEMA").expect("没有获取到环境变量 DB_SCHEMA");

        manager
            .create_table(
                Table::create()
                    .table(TableRef::SchemaTable(
                        Alias::new(schema).into_iden(),
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
                            .custom(Alias::new("entity_status"))
                            .not_null()
                            .default(Expr::cust("'ACTIVE'::entity_status"))
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    Phone,
    PasswordHash,
    DisplayName,
    Role,
    EntSt,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    UpdatedReason,
}
