use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PositionsFile {
    #[serde(skip_serializing, skip_deserializing)]
    pub path: String,
    pub created_datetime: String,
    pub modified_datetime: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub position: Vec<Position>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Position {
    /// full system path to the file
    pub file_path: String,
    /// Unique identifier for the file.
    /// Will be Inode on linux and File ID on Windows.
    pub file_id: u64,
    /// number of bytes that have been read.
    pub bytes_read: u64,
}

impl Position {
    pub fn new(file_path: &str, file_id: u64, bytes_read: u64) -> Self {
        Self {
            file_path: file_path.to_string(),
            file_id,
            bytes_read,
        }
    }
}
