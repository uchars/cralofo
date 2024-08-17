use log::trace;

use crate::models::logs::Logs;

pub async fn publish_logs(url: &str, logs: Logs) {
    trace!("publishing {:?} logs to '{}'", logs, url);
    let client = reqwest::Client::new();
    if let Err(err) = client.post(url).json(&logs).send().await {
        log::error!("{}", err.to_string());
    }
}
