mod config;
mod controller;
mod entity;
mod env;
mod grpc; // grpc 模块
mod mapper;
mod route;
mod service;
mod r#type;
mod util;

use crate::{
    config::swagger_ui::doc::build_openapi,
    env::{APP_PORT, DB_HOST, DB_NAME, DB_PASSWORD, DB_PORT, DB_SCHEMA, DB_USER},
    grpc::server::start_grpc_server,
    route::build_api_route,
};
use actix_web::{web, App, HttpServer};
use common_config::{configure, init_logger, load_env};
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

    let openapi = build_openapi();
    let db_data = web::Data::new(db);

    // --- gRPC server ---
    let grpc_server = tokio::spawn(async move {
        start_grpc_server("[::1]:50051").await;
    });

    // --- HTTP server ---
    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(configure(openapi.clone(), build_api_route()))
    })
    .bind(("0.0.0.0", *APP_PORT))?
    .run();

    // 并行运行两个 server
    tokio::select! {
        _ = grpc_server => {
            println!("gRPC server task exited");
        }
        res = http_server => {
            res?; // 如果 HTTP server 出错，直接返回 Err
        }
    }

    Ok(())
}
