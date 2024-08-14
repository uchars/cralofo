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
