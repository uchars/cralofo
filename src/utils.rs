#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(windows)]
use winapi::shared::ntdef::NULL;
#[cfg(windows)]
use winapi::um::fileapi::{GetFileInformationByHandle, FILE_ATTRIBUTE_NORMAL};
#[cfg(windows)]
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
#[cfg(windows)]
use winapi::um::winbase::CreateFileW;
#[cfg(windows)]
use winapi::um::winnt::HANDLE;
#[cfg(windows)]
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, OPEN_EXISTING};

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
    use std::ffi::OsStr;
    use std::os::windows::prelude::*;
    use std::ptr;

    let path = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let handle: HANDLE = unsafe {
        CreateFileW(
            path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
    }

    let mut file_info = unsafe { std::mem::zeroed() };
    let result = unsafe { GetFileInformationByHandle(handle, &mut file_info) };

    unsafe { winapi::um::handleapi::CloseHandle(handle) };

    if result == 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok((file_info.nFileIndexHigh as u64) << 32 | file_info.nFileIndexLow as u64)
}
