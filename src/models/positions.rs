use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PositionsFile {
    pub created_datetime_str: String,
    pub modified_datetime_str: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub position: Vec<Position>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Position {
    /// full system path to the file
    pub file_path: String,
    /// Unique identifier for the file.
    /// Will be Inode on linux and File ID on Windows.
    pub file_id: String,
    /// Last line that has been read.
    pub line: u64,
}
