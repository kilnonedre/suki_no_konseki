use std::sync::Once;

use env_logger::{Builder, Env};

pub fn init_logger() {
    init_logger_with_level(None);
}

pub fn init_logger_with_level(level: Option<&str>) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let default_level = level.unwrap_or("warn");
        let env = Env::default().filter_or("RUST_LOG", default_level);
        Builder::from_env(env).format_timestamp_millis().init();
    });
}
