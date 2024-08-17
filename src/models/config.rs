use core::fmt;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub settings: Settings,
    pub files: FileConfigs,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileConfigs(pub Vec<FileConfig>);

#[derive(Debug, Deserialize, Clone)]
pub struct FileConfig {
    pub positions_file: String,
    pub path: String,
    pub labels: HashMap<String, String>,
    pub file_regex: String,
    #[serde(default = "default_forward_frequency_ms")]
    pub forward_frequency_ms: u32,
    #[serde(default = "default_buffersize")]
    pub buffer_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: String,
    #[serde(default = "default_loglevel")]
    pub log_level: String,
    #[serde(default = "default_scan_existing")]
    pub scan_existing: bool,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n{}", self.settings, self.files)
    }
}

impl fmt::Display for FileConfigs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Files\n")?;
        for (pos, file) in self.0.iter().enumerate() {
            write!(
                f,
                "  File #{}\n    Path: {}\n    File Regex: {}\n    Forward Frequency: {}ms\n    Buffer Size (Byte): {}\n",
                pos,
                file.path,
                file.file_regex,
                file.forward_frequency_ms,
                file.buffer_size
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Settings:\n  adress: {}", self.server,)
    }
}

fn default_loglevel() -> String {
    "warn".to_string()
}

fn default_buffersize() -> u32 {
    1000000
}

fn default_forward_frequency_ms() -> u32 {
    5000
}

fn default_scan_existing() -> bool {
    false
}
