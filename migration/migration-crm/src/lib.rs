pub use sea_orm_migration::prelude::*;
mod env;
mod iden;

mod m20250917_092058_add_entity_status_enum;
mod m20250917_092059_add_audit_status_enum;
mod m20250925_073218_add_users_table;
mod m20250928_015340_add_user_profiles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250917_092058_add_entity_status_enum::Migration),
            Box::new(m20250917_092059_add_audit_status_enum::Migration),
            Box::new(m20250925_073218_add_users_table::Migration),
            Box::new(m20250928_015340_add_user_profiles_table::Migration),
        ]
    }
}
