mod config;
mod controller;
mod entity;
mod env;
mod mapper;
mod route;
mod service;
mod r#type;
mod util;

use crate::{
    config::swagger_ui_config::{doc::build_openapi, ui::configure},
    env::{APP_PORT, DB_HOST, DB_NAME, DB_PASSWORD, DB_PORT, DB_SCHEMA, DB_USER},
};
use actix_web::{web, App, HttpServer};
use common_config::{init_logger, load_env};
use migration_crm::{Migrator, MigratorTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    load_env();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        *DB_USER, *DB_PASSWORD, *DB_HOST, *DB_PORT, *DB_NAME
    );

    let db = common_db::init_db_with_schema(&db_url, &*DB_SCHEMA).await;

    Migrator::up(&db, None).await.expect("migration failed");

    // ……后面保持不变
    let openapi = build_openapi();
    let db_data = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(configure(openapi.clone()))
    })
    .bind(("0.0.0.0", *APP_PORT))?
    .run()
    .await
}
