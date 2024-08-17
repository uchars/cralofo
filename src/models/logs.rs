use std::time::SystemTime;

pub struct FileRead {
    /// full lines that were read.
    pub lines: Vec<String>,
    /// new starting position for the next read.
    /// new_pos = start_pos + bytes_read
    pub new_pos: u64,
}

#[derive(Debug)]
pub struct Logs {
    pub system_time_nanoseconds: u128,
    pub logs: Vec<String>,
}

impl Logs {
    pub fn new(logs: Vec<String>) -> Self {
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        Self {
            system_time_nanoseconds: duration_since_epoch.as_nanos(),
            logs,
        }
    }
}
