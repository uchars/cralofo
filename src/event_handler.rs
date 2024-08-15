use std::path::Path;

use log::{debug, error, trace};
use notify::{RecommendedWatcher, Watcher};

use crate::{
    api::publish_logs,
    models::{
        config::Settings,
        logs::Logs,
        positions::{Position, PositionsFile},
    },
    utils::get_file_inode,
};

pub struct EventHandler {
    pub settings: Settings,
    pub positions: PositionsFile,
}

impl EventHandler {
    pub fn new(settings: Settings, positions: PositionsFile) -> Self {
        Self {
            settings,
            positions,
        }
    }

    /// Watch events for a given file.
    /// Will keep the positions file up to date and publish log information to the API.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
        trace!("watch");
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
        watcher.watch(path.as_ref(), notify::RecursiveMode::NonRecursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    if let Err(err) = self.handle_file_event(&event) {
                        error!("{}", err);
                    }
                }
                Err(error) => error!("Error: {error:?}"),
            }
        }

        Ok(())
    }

    /// This method forwards a file event to it's handler method.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn handle_file_event(&mut self, event: &notify::Event) -> Result<(), &'static str> {
        trace!("Change: {event:?}");
        match event.kind {
            notify::EventKind::Access(_) => self.handle_file_access(&event)?,
            notify::EventKind::Create(notify::event::CreateKind::File) => {
                self.handle_create_file(&event)?
            }
            notify::EventKind::Modify(notify::event::ModifyKind::Name(
                notify::event::RenameMode::Both,
            )) => self.handle_rename_file(&event)?,
            notify::EventKind::Remove(_) => self.handle_remove_file(&event)?,
            _ => {}
        };
        Ok(())
    }

    /// This helper method handles the create file event.
    /// # Errors
    ///
    /// This function will return an error if .
    fn handle_create_file(&mut self, event: &notify::Event) -> Result<(), &'static str> {
        if event.paths.is_empty() {
            return Err("create file event did not contain file.");
        }
        let path = match event.paths[0].as_os_str().to_str() {
            Some(path) => path,
            None => return Err("could not convert path to string"),
        };
        let inode = match get_file_inode(path) {
            Ok(inode) => inode,
            Err(_) => return Err("could not get inode file"),
        };
        let position = Position::new(path.to_string(), inode, 0, 0);
        self.positions.add_position(&position);
        if let Err(err) = self.positions.write() {
            error!("Write failed: {}", err);
        }
        Ok(())
    }

    /// This helper method handles the rename file event.
    /// It usally appears when the logger uses a rotating file sync (renaming other log files before creating new one).
    /// # Note
    /// Let's say there are 2 log files `foo.log` and `foo.1.log`.
    /// once `foo.log` is full the rotating filesink will rename `foo.1.log` -> `foo.2.log` and `foo.log` -> `foo.1.log`
    /// before creating the new `foo.log`.
    ///
    /// This means that the positions file needs to be updated twice.
    /// # Errors
    ///
    /// This function will return an error if .
    fn handle_rename_file(&mut self, event: &notify::Event) -> Result<(), &'static str> {
        if event.paths.len() < 2 {
            return Err("rename event contained less than 2 elements for some reason.");
        }
        let to = match event.paths[1].as_os_str().to_str() {
            Some(path) => path,
            None => return Err("could not convert path to string"),
        };
        // get inode for old and new file
        let inode = match get_file_inode(to) {
            Ok(inode) => inode,
            Err(_) => return Err("could not get inode file"),
        };
        self.positions.rename_position(inode, to);
        if let Err(e) = self.positions.write() {
            error!("failed to write: {}", e);
        }
        Ok(())
    }

    /// This helper method will handle the remove file event.
    /// It makes sure the positions file is updated.
    ///
    /// # Arguments
    ///
    /// * `event` - file remove event.
    fn handle_remove_file(&mut self, event: &notify::Event) -> Result<(), &'static str> {
        // if folder removed, also remove all files inside folder.
        debug!("removed {:?}", event.paths);
        if event.paths.is_empty() {
            return Err("create file event did not contain file.");
        }
        let path = match event.paths[0].as_os_str().to_str() {
            Some(path) => path,
            None => return Err("could not convert path to string"),
        };
        // need to remove based on file name, since file no longer exists.
        self.positions
            .remove_position(|other| other.file_path == path);
        if let Err(e) = self.positions.write() {
            error!("failed to write: {}", e);
        }

        Ok(())
    }

    /// This function will return an error if .
    /// This helper method handles the file access event.
    /// It mostly just filters out the irrelevant events which are not relevant for crab_crib's featureset.
    ///
    /// # Arguments
    ///
    /// * `event` - file access event.
    fn handle_file_access(&self, event: &notify::Event) -> Result<(), &'static str> {
        // only handling file write event.
        match event.kind {
            notify::EventKind::Access(notify::event::AccessKind::Close(
                notify::event::AccessMode::Write,
            )) => self.handle_file_write(event)?,
            _ => {}
        }
        Ok(())
    }

    /// This helper method handles the file write event.
    /// More specifically, when the file has been written to and closed.
    ///
    /// After reading the new data that was added to the file it will publish the logs to the API.
    ///
    /// # Arguments
    ///
    /// * `event` - file system event.
    ///
    /// # Errors
    ///
    /// - write event did not have a path.
    ///
    fn handle_file_write(&self, event: &notify::Event) -> Result<(), &'static str> {
        if event.paths.is_empty() {
            return Err("file write event received without file info.");
        }
        let path = event.paths[0].as_os_str();
        debug!("handling file write for {:?}", path);
        // TODO: read logs from file
        // send logs to API
        publish_logs(&self.settings.server, Logs::new()?);
        Ok(())
    }
}
