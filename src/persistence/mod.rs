mod ron_file_persistence;

use crate::persistence::ron_file_persistence::RonFilePersistence;
use crate::task::Task;
use anyhow::Result;
use std::path::Path;

pub trait Persistence {
    fn save(&self, data: &[Task]) -> Result<()>;
    fn load(&self) -> Result<Vec<Task>>;
}

pub fn get_persistence(location: impl AsRef<Path>) -> Box<dyn Persistence> {
    log::trace!("Creating persistence (RonFilePersistence)");
    Box::new(RonFilePersistence::new(location))
}
