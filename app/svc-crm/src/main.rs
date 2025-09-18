mod config;
mod controller;
mod entity;
mod route;
mod service;
mod r#type;

use std::env;

use actix_web::{web, App, HttpServer};
use common_config::{init_logger, load_env};
use internal_clients::PolicyClient;
use migration_crm::{Migrator, MigratorTrait};

use crate::config::swagger_ui_config::{doc::build_openapi, ui::configure};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    load_env();

    let db_url = build_database_url();
    let policy_base =
        env::var("POLICY_BASE_URL").unwrap_or_else(|_| "http://svc-policy:8082".into());

    let db = common_db::connect(&db_url).await;

    Migrator::up(&db, None).await.expect("error");

    let policy = PolicyClient::new(policy_base).expect("build PolicyClient");

    // 生成 OpenAPI（在 api::mod.rs 里定义）
    let openapi = build_openapi();

    let db_data = web::Data::new(db);
    let policy_data = web::Data::new(policy);

    HttpServer::new(move || {
        App::new()
            // .wrap(RequestId)
            // .wrap(cors_permissive())
            .app_data(db_data.clone())
            .app_data(policy_data.clone())
            // 统一在 api 模块里注册路由 + Swagger UI
            .configure(configure(openapi.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn build_database_url() -> String {
    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_default();
    let name = env::var("DB_NAME").expect("DB_NAME must be set");

    if password.is_empty() {
        format!("postgres://{}@{}:{}/{}", user, host, port, name)
    } else {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, name
        )
    }
}
