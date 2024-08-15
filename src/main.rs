use std::{
    fs,
    thread::{self, JoinHandle},
};

use log::{debug, error, info, trace, warn};
use models::{
    config::{Config, FileConfig, Settings},
    positions::PositionsFile,
};
use utils::file_exists;

pub mod api;
pub mod event_handler;
pub mod models;
pub mod positions_manager;
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
        // get positions information for path, if exists or create new positions file.
        let mut positions_file = match parse_positions_file(&file_conf.positions_file) {
            Ok(p) => p,
            Err(e) => {
                error!("{}", e);
                PositionsFile::default()
            }
        };
        // write the created TOML struct to the disk
        positions_file.set_path(&file_conf.positions_file);
        match positions_file.write() {
            Ok(_) => debug!("file updated '{}'", file_conf.positions_file),
            Err(e) => warn!("create file failed. reason: {}", e),
        };
        let mut handler = event_handler::EventHandler {
            settings,
            positions: positions_file,
        };
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

fn parse_positions_file(path: &str) -> Result<PositionsFile, String> {
    let positions_file: PositionsFile;
    if !file_exists(path) {
        debug!("creating positions struct '{}'", path);
        positions_file = PositionsFile::new(path);
    } else {
        debug!("using existing positions file '{}'", path);
        let file_str = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        };
        positions_file = match toml::de::from_str(&file_str) {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        };
    }
    Ok(positions_file)
}
