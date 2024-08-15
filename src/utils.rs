use std::{
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::Utc;
use log::LevelFilter;

pub fn set_log_level(level: String) {
    let level_filter = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    env_logger::Builder::new().filter(None, level_filter).init();
}

pub fn get_unix_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_millis()
}

pub fn get_datetime_str() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn file_exists(file_name: &str) -> bool {
    Path::new(file_name).exists()
}
