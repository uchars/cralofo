use std::{
    fs,
    thread::{self, JoinHandle},
};

use log::{error, info, trace, warn};
use models::config::{Config, FileConfig, Settings};

pub mod api;
pub mod event_handler;
pub mod models;
pub mod utils;

pub fn main() {
    let config_content = fs::read_to_string("config.toml").expect("unable to find config.toml");
    let config: Config = toml::de::from_str(&config_content).expect("could not parse config file");
    let mut handles = vec![];

    utils::set_log_level(config.settings.log_level.clone());

    info!("Using the following configuration {}", config);

    for i in 0..config.files.0.len() {
        let file = config.files.0[i].clone();
        let settings = config.settings.clone();
        handles.push(start_thread(settings, file));
    }

    join_threads(handles)
}

fn start_thread(settings: Settings, file_conf: FileConfig) -> JoinHandle<()> {
    info!("starting thread for path '{}'...", file_conf.path);
    thread::spawn(move || {
        info!("thread for '{}' started", file_conf.path);
        let handler = event_handler::EventHandler { settings };
        if let Err(error) = handler.watch(file_conf.path.clone()) {
            error!(
                "({}) Error: {:?} {:?}",
                file_conf.path.clone(),
                error.kind,
                error.paths
            );
        }
    })
}

fn join_threads(handles: Vec<JoinHandle<()>>) {
    trace!("joining {} threads", handles.len());
    let mut i = 0;
    for handle in handles {
        match handle.join() {
            Ok(_) => info!("stopped thread #{}", i),
            Err(_) => warn!("could not stop thread #{}.", i),
        }
        i += 1;
    }
}
