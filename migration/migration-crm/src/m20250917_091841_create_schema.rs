use std::env;

use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_connection().get_database_backend();
        let schema = env::var("DB_SCHEMA").expect("没有获取到环境变量 DB_SCHEMA");
        let sql = format!("CREATE SCHEMA IF NOT EXISTS {}", schema);
        manager
            .get_connection()
            .execute(Statement::from_string(backend, sql))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_connection().get_database_backend();

        manager
            .get_connection()
            .execute(Statement::from_string(
                backend,
                "DROP SCHEMA IF EXISTS app CASCADE".to_owned(),
            ))
            .await?;
        Ok(())
    }
}
