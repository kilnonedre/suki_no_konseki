pub use sea_orm_migration::prelude::*;

mod m20250917_091841_create_schema;
mod m20250917_092058_create_entity_status_enum;
mod m20250925_073218_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250917_091841_create_schema::Migration),
            Box::new(m20250917_092058_create_entity_status_enum::Migration),
            Box::new(m20250925_073218_create_user_table::Migration),
        ]
    }
}
