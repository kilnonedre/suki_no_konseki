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
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(50)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(100)
                            .unique_key()
                            .null(),
                    )
                    .col(ColumnDef::new(Users::Phone).string_len(20).null())
                    .col(ColumnDef::new(Users::PasswordHash).text().not_null())
                    .col(ColumnDef::new(Users::DisplayName).string_len(100).null())
                    .col(ColumnDef::new(Users::Role).string_len(30).not_null())
                    .col(ColumnDef::new(Users::EntSt).string_len(20).not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Users::CreatedBy).big_integer().null())
                    .col(ColumnDef::new(Users::UpdatedBy).big_integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

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
}
