use std::env;

use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use sea_orm_migration::sea_query::Alias;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = env::var("DB_SCHEMA").expect("没有获取到环境变量 DB_SCHEMA");

        manager
            .create_type(
                Type::create()
                    .as_enum((Alias::new(&schema), Alias::new("entity_status")))
                    .values([
                        Alias::new("ACTIVE"),
                        Alias::new("INACTIVE"),
                        Alias::new("DELETED"),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = env::var("DB_SCHEMA").expect("没有获取到环境变量 DB_SCHEMA");

        manager
            .drop_type(
                Type::drop()
                    .name((Alias::new(&schema), Alias::new("entity_status")))
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
