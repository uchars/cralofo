use std::path::Path;

use log::{debug, error, trace};
use notify::{RecommendedWatcher, Watcher};

use crate::{
    api::publish_logs,
    models::{config::Settings, logs::Logs},
};

pub struct EventHandler {
    pub settings: Settings,
}

impl EventHandler {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Watch events for a given file.
    /// Will keep the positions file up to date and publish log information to the API.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn watch<P: AsRef<Path>>(&self, path: P) -> notify::Result<()> {
        trace!("watch");
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
        watcher.watch(path.as_ref(), notify::RecursiveMode::NonRecursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    let _ = self.handle_file_event(&event);
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
    fn handle_file_event(&self, event: &notify::Event) -> Result<(), &'static str> {
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
    fn handle_create_file(&self, event: &notify::Event) -> Result<(), &'static str> {
        if event.paths.is_empty() {
            return Err("create file event did not contain file.");
        }
        debug!("created file {:?}", event.paths[0].as_os_str());
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
    fn handle_rename_file(&self, event: &notify::Event) -> Result<(), &'static str> {
        let from = event.paths[0].as_os_str();
        let to = event.paths[1].as_os_str();
        debug!("file renamed {:?} -> {:?}", from, to);
        Ok(())
    }

    /// This helper method will handle the remove file event.
    /// It makes sure the positions file is updated.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn handle_remove_file(&self, event: &notify::Event) -> Result<(), &'static str> {
        // if folder removed, also remove all files inside folder.
        debug!("removed {:?}", event.paths);
        Ok(())
    }

    /// This helper method handles the file access event.
    /// It mostly just filters out the irrelevant events which are not relevant for crab_crib's featureset.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
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
    /// # Errors
    ///
    /// This function will return an error if
    /// - The event does not contain valid path info.
    /// - File cannot ben read.
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
