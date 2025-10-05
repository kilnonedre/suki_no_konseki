use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use sea_orm_migration::sea_query::Alias;

use crate::env::DB_SCHEMA;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum((Alias::new(&*DB_SCHEMA), Alias::new("entity_status")))
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
        manager
            .drop_type(
                Type::drop()
                    .name((Alias::new(&*DB_SCHEMA), Alias::new("entity_status")))
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
