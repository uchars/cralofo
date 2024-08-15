use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PositionsFile {
    pub created_unix_millis: u64,
    pub modified_unix_millis: u64,
    pub position: Positions,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Positions(pub Vec<Position>);

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    /// full system path to the file
    pub file_path: String,
    /// Unique identifier for the file.
    /// Will be Inode on linux and File ID on Windows.
    pub file_id: String,
    /// Last line that has been read.
    pub line: u128,
}
