use log::trace;
use reqwest::Response;

use crate::models::logs::Logs;

pub async fn publish_logs(url: &str, logs: Logs) -> Result<Response, reqwest::Error> {
    trace!("publishing {:?} logs to '{}'", logs, url);
    let client = reqwest::Client::new();
    client.post(url).json(&logs).send().await
}
