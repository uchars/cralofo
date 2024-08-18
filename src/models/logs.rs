use std::{str::FromStr, time::SystemTime};

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub struct FileRead {
    /// full lines that were read.
    pub lines: Vec<String>,
    /// new starting position for the next read.
    /// new_pos = start_pos + bytes_read
    pub new_pos: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logs {
    pub system_time_nanoseconds: u128,
    pub logs: Vec<Log>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub message: String,
    pub timestamp_nanos: i64,
}

impl Logs {
    pub fn from_lines(lines: Vec<String>) -> Self {
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let mut logs: Vec<Log> = Vec::new();
        for l in lines {
            Log::from_str(&l).map(|log| logs.push(log));
        }
        Self {
            system_time_nanoseconds: duration_since_epoch.as_nanos(),
            logs,
        }
    }
}

impl Log {
    fn new(timestamp_nanos: i64, message: &str) -> Self {
        Log {
            message: message.to_string(),
            timestamp_nanos,
        }
    }

    pub fn from_str(line: &str) -> Option<Self> {
        let re_time = Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z").unwrap();
        let time_nanos: i64 = re_time
            .find(line)
            .map(|m| m.as_str())
            .map(|m| DateTime::<Utc>::from_str(m))?
            .map(|x| x.timestamp_nanos_opt())
            .ok()??;
        Some(Log::new(time_nanos, line))
    }
}
