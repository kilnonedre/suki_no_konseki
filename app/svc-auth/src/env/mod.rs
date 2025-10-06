use std::env;

use once_cell::sync::Lazy;
use uuid::Uuid;

pub static DB_SCHEMA: Lazy<String> =
    Lazy::new(|| env::var("DB_SCHEMA").expect("❌ 必须设置环境变量 DB_SCHEMA"));

pub static APP_PORT: Lazy<u16> = Lazy::new(|| {
    let val = env::var("APP_PORT").expect("❌ 必须设置环境变量 APP_PORT");
    val.parse::<u16>()
        .expect("❌ APP_PORT 必须是 0~65535 的数字")
});

pub static DB_HOST: Lazy<String> =
    Lazy::new(|| env::var("DB_HOST").expect("❌ 必须设置环境变量 DB_HOST"));

pub static DB_PORT: Lazy<String> =
    Lazy::new(|| env::var("DB_PORT").expect("❌ 必须设置环境变量 DB_PORT"));

pub static DB_USER: Lazy<String> =
    Lazy::new(|| env::var("DB_USER").expect("❌ 必须设置环境变量 DB_USER"));

pub static DB_PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("DB_PASSWORD").expect("❌ 必须设置环境变量 DB_PASSWORD"));

pub static DB_NAME: Lazy<String> =
    Lazy::new(|| env::var("DB_NAME").expect("❌ 必须设置环境变量 DB_NAME"));

pub static SYS_ID: Lazy<Uuid> = Lazy::new(|| {
    let s = env::var("SYS_ID").expect("❌ 必须设置环境变量 SYS_ID");
    Uuid::parse_str(&s).expect("❌ SYS_ID 必须是合法的 UUID")
});
