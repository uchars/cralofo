use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::models::positions::{Position, PositionsFile};

impl PositionsFile {
    pub fn new() -> Self {
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(0)
            .as_millis();

        PositionsFile {
            created_unix_millis: created,
            modified_unix_millis: created,
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
        self.position.0.push(position);
        update();
    }

    /// Update a specific table in the array based on the unique file id.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique file id of the element
    /// * `new` - Struct containing the updated values.
    pub fn update_position(&mut self, id: String, new: &Position) -> Result<(), ()> {
        let element = match self.position.0.iter().find(|&other| other.file_id == id) {
            Some(e) => e,
            None => return Err(()),
        };
        element.file_path = new.file_path;
        update();
        Ok(())
    }

    /// Internal helper method to update some member fields.
    fn update(&mut self) {
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(0)
            .as_millis();
        self.modified_unix_millis = created;
    }

    /// Write the TOML struct in its current state to a specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path (including filename) where the .toml will be created.
    pub fn write(&self, path: &str) -> Result<(), &'static str> {
        let toml = match toml::to_string(&self) {
            Ok(t) => t,
            Err(e) => return Err("could not serialize toml struct."),
        };
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
        file.write_all(toml.as_bytes())
    }
}
