pub use sea_orm_migration::prelude::*;
mod env;
mod iden;

mod m20251013_041426_add_entity_status_enum;
mod m20251013_041443_add_audit_status_enum;
mod m20251013_041522_add_refresh_tokens_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251013_041426_add_entity_status_enum::Migration),
            Box::new(m20251013_041443_add_audit_status_enum::Migration),
            Box::new(m20251013_041522_add_refresh_tokens_table::Migration),
        ]
    }
}
