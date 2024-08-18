#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
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

pub fn get_file_inode(path: &Path) -> Option<u64> {
    if let Ok(inode) = get_inode(path) {
        return Some(inode);
    }
    None
}

#[cfg(unix)]
fn get_inode(path: &Path) -> std::io::Result<u64> {
    let metadata = path.metadata()?;
    Ok(metadata.ino())
}

#[cfg(windows)]
fn get_inode(path: &Path) -> std::io::Result<u64> {
    use std::{fs::OpenOptions, os::windows::fs::OpenOptionsExt};
    use std::{mem, os::windows::prelude::*};
    use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS;
    use windows_sys::Win32::{
        Foundation::HANDLE,
        Storage::FileSystem::{GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION},
    };
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;

    unsafe {
        let mut info: BY_HANDLE_FILE_INFORMATION = mem::zeroed();
        let ret = GetFileInformationByHandle(file.as_raw_handle() as HANDLE, &mut info);
        if ret == 0 {
            return Err(std::io::Error::last_os_error());
        };

        Ok(((info.nFileIndexHigh as u64) << 32) | (info.nFileIndexLow as u64))
    }
}
