use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect(db_url: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    Database::connect(opt).await.expect("connect db")
}
