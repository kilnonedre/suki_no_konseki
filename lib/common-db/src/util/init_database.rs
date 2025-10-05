use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};

pub async fn connect_with_search_path(
    db_url: &str,
    search_path: Option<&str>,
) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(db_url.to_owned());
    if let Some(sp) = search_path {
        opt.set_schema_search_path(sp.to_string()); // 只有在传入时才设置
    }
    opt.max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    Database::connect(opt).await.expect("connect db")
}

pub async fn init_db_with_schema(db_url: &str, schema: &str) -> DatabaseConnection {
    let bootstrap = connect_with_search_path(db_url, None).await;

    let be = bootstrap.get_database_backend();
    bootstrap
        .execute(Statement::from_string(
            be,
            format!("CREATE SCHEMA IF NOT EXISTS {schema}"),
        ))
        .await
        .expect("create schema");

    drop(bootstrap);
    connect_with_search_path(db_url, Some(&format!("{schema}, public"))).await
}
