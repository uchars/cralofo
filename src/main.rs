use std::{
    fs,
    thread::{self, JoinHandle},
};

use log::{debug, error, info, trace, warn};
use models::{
    config::{Config, FileConfig, Settings},
    positions::{Position, PositionsFile},
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
        let mut positions_file =
            parse_positions_file(&file_conf.positions_file).unwrap_or_default();
        // add position just for fun.
        positions_file.add_position(&Position {
            file_path: "foo".to_string(),
            file_id: "bar".to_string(),
            line: 128,
        });
        // write the created TOML struct to the disk
        match positions_file.write(&file_conf.positions_file) {
            Ok(_) => debug!("file updated '{}'", file_conf.positions_file),
            Err(e) => warn!("create file failed. reason: {}", e),
        };
        let handler = event_handler::EventHandler {
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

fn parse_positions_file(path: &str) -> Result<PositionsFile, ()> {
    let positions_file: PositionsFile;
    if !file_exists(path) {
        debug!("creating positions struct '{}'", path);
        positions_file = PositionsFile::new();
    } else {
        debug!("using existing positions file '{}'", path);
        let file_str = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_) => return Err(()),
        };
        positions_file = match toml::de::from_str(&file_str) {
            Ok(d) => d,
            Err(_) => return Err(()),
        };
    }
    Ok(positions_file)
}
