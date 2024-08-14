use log::{info, trace};

use crate::models::logs::Logs;

pub fn publish_logs(url: &str, logs: Logs) {
    info!("publishing {:?} logs to '{}'", logs, url);
    trace!("publishing logs {:?}", logs,);
}
