use std::{fs::File, io::Write};

use log::{debug, info};

use crate::{
    models::positions::{Position, PositionsFile},
    utils::get_datetime_str,
};

impl PositionsFile {
    pub fn new(path: &str) -> Self {
        debug!("creating positions file for path '{}'", path);
        let created = get_datetime_str();
        PositionsFile {
            path: path.to_string(),
            created_datetime_str: created.clone(),
            modified_datetime_str: created.clone(),
            position: Vec::new(),
        }
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

    /// Update the file name of position based on it's id.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique file id of the element
    /// * `new_path` - New path of the file.
    pub fn rename_position(&mut self, id: u64, new_path: &str) {
        match self.position.iter().position(|other| other.file_id == id) {
            Some(index) => {
                debug!(
                    "update file path '{}' -> '{}'",
                    self.position[index].file_path, new_path
                );
                self.position[index].file_path = new_path.to_string();
                self.update();
            }
            None => {
                debug!("could not find position with id '{}'", id);
            }
        };
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
        match self.position.iter().position(predicate) {
            Some(index) => {
                debug!("remove position");
                self.position.remove(index);
                self.update();
            }
            None => {
                debug!("could not find position based on predicate");
            }
        };
    }

    /// Internal helper method to update some member fields.
    fn update(&mut self) {
        self.modified_datetime_str = get_datetime_str();
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
}
