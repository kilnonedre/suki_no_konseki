use std::env;

use once_cell::sync::Lazy;

pub static DB_SCHEMA: Lazy<String> =
    Lazy::new(|| env::var("DB_SCHEMA").expect("❌ 必须设置环境变量 DB_SCHEMA"));
