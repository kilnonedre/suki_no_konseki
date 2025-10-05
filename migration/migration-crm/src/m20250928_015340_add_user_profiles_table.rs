use sea_orm_migration::prelude::*;

use crate::{
    env::DB_SCHEMA,
    iden::{user_profiles::UserProfiles, users::Users},
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
                        UserProfiles::Table.into_iden(),
                    ))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfiles::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .comment("与 users.id 共享主键的用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::RealName)
                            .string_len(50)
                            .null()
                            .comment("真实姓名"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::IdCardNo)
                            .string_len(32)
                            .null()
                            .comment("身份证号码"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::Gender)
                            .string_len(10)
                            .null()
                            .comment("性别"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::Birthday)
                            .date()
                            .null()
                            .comment("生日"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::Address)
                            .string_len(200)
                            .null()
                            .comment("联系地址"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::CreatedBy)
                            .uuid()
                            .null()
                            .comment("创建人 ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UpdatedBy)
                            .uuid()
                            .null()
                            .comment("更新人 ID"),
                    )
                    .col(
                        ColumnDef::new(UserProfiles::UpdatedReason)
                            .text()
                            .null()
                            .comment("更新原因"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_profiles_id__users_id")
                    .from_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UserProfiles::Table.into_iden(),
                    ))
                    .from_col(UserProfiles::Id)
                    .to_tbl(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        Users::Table.into_iden(),
                    ))
                    .to_col(Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_user_profiles_id__users_id")
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UserProfiles::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(TableRef::SchemaTable(
                        Alias::new(&*DB_SCHEMA).into_iden(),
                        UserProfiles::Table.into_iden(),
                    ))
                    .to_owned(),
            )
            .await
    }
}
