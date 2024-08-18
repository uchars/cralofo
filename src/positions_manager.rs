use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use log::{debug, info, trace};

use crate::{
    models::positions::{Position, PositionsFile},
    utils::{get_datetime_str, get_file_inode},
};

impl PositionsFile {
    pub fn new(path: &str) -> Self {
        debug!("creating positions file for path '{}'", path);
        let created = get_datetime_str();
        PositionsFile {
            path: path.to_string(),
            created_datetime: created.clone(),
            modified_datetime: created.clone(),
            position: Vec::new(),
        }
    }

    pub fn init(&mut self, log_dir: &str) {
        self.filename_update(log_dir);
        self.find_and_add();
        self.cleanup();
    }

    /// Add a new table element to the table vector.
    ///
    /// # Arguments
    ///
    /// * `position` - new table to add to the array
    ///
    /// # Examples
    ///
    /// Before:
    /// ```toml
    /// [[position]]
    /// ...
    /// ```
    ///
    /// `add_position(...)`
    ///
    /// After:
    /// ```toml
    /// [[position]]
    /// # Some data
    ///
    /// [[position]]
    /// # Some new data
    /// ```
    pub fn add_position(&mut self, position: &Position) {
        if !self
            .position
            .iter()
            .any(|other| other.file_id == position.file_id)
        {
            self.position.push(position.clone());
            self.update();
        } else {
            info!(
                "File already contains entry for id {}. skipping...",
                position.file_id
            );
        }
    }

    pub fn find<F>(&self, predicate: F) -> Option<&Position>
    where
        F: Fn(&&Position) -> bool,
    {
        self.position.iter().find(predicate)
    }

    pub fn update_bytes_read(&mut self, id: u64, new_byte_pos: u64) {
        self.position
            .iter()
            .position(|other| other.file_id == id)
            .map(|idx| {
                log::trace!(
                    "updating byte pos {} -> {}",
                    self.position[idx].bytes_read,
                    new_byte_pos
                );
                self.position[idx].bytes_read = new_byte_pos;
                self.update()
            });
    }

    /// Update the file name of position based on it's id.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique file id of the element
    /// * `new_path` - New path of the file.
    pub fn rename_position(&mut self, id: u64, new_path: &str) {
        trace!("attempting to rename file({}) -> {}", id, new_path);
        self.position
            .iter()
            .position(|other| other.file_id == id)
            .map(|idx| {
                trace!(
                    "({}) update file path '{}' -> '{}'",
                    id,
                    self.position[idx].file_path,
                    new_path
                );
                self.position[idx].file_path = new_path.to_string();
                self.update();
            });
    }

    /// Remove a table from the TOML struct.
    ///
    /// # Arguments
    ///
    /// * `id` - id of the table to remove from the TOML
    ///
    /// # Examples
    ///
    /// Before:
    /// ```toml
    /// [[position]]
    /// file_id = 12
    ///
    /// [[position]]
    /// file_id = 42
    /// ```
    ///
    /// `remove_position(12)`
    ///
    /// After:
    /// ```toml
    /// [[position]]
    /// file_id = 42
    /// ```
    pub fn remove_position<F>(&mut self, predicate: F)
    where
        F: Fn(&Position) -> bool,
    {
        self.position.iter().position(predicate).map(|idx| {
            debug!("remove position");
            self.position.remove(idx);
            self.update();
        });
    }

    /// Internal helper method to update some member fields.
    fn update(&mut self) {
        self.modified_datetime = get_datetime_str();
    }

    /// Write the TOML struct in its current state to a specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path (including filename) where the .toml will be created.
    pub fn write(&self) -> Result<(), String> {
        debug!("write TOML to '{}'", self.path);
        let toml = toml::to_string(&self.clone())
            .map_err(|e| std::format!("could not serialize struct ({})", e.to_string()))?;
        let mut file = File::create(&self.path)
            .map_err(|e| std::format!("could not create file ({})", e.to_string()))?;
        file.write_all(toml.as_bytes())
            .map_err(|e| std::format!("could not write to file ({})", e.to_string()))
    }

    pub fn set_path(&mut self, path: &str) {
        debug!("set path {}", path);
        self.path = path.to_string();
    }

    /// Remove all positions which no longer exist.
    fn cleanup(&mut self) {
        self.position
            .retain(|p| Path::exists(Path::new(&p.file_path)));
    }

    /// Check if inode still matches filename, if not update.
    fn filename_update(&mut self, log_dir: &str) {
        if let Ok(dir_files) = fs::read_dir(log_dir) {
            // get files and their inode and update filename
            dir_files
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let path = e.path();
                    let inode = get_file_inode(&path)?;
                    path.to_str().map(|s| (s.to_string(), inode))
                })
                .collect::<Vec<(String, u64)>>()
                .iter()
                .for_each(|(fname, inode)| {
                    debug!("checking file ({}, {})", fname, inode);
                    self.rename_position(*inode, fname);
                });
        } else {
            log::warn!("could not get files in dir '{}'", log_dir);
        }
    }

    /// Search directory for files which match regex and add them.
    fn find_and_add(&mut self) {}
}
