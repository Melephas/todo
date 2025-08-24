mod ron_file_persistence;

use std::path::Path;
use crate::task::Task;
use anyhow::Result;
use crate::persistence::ron_file_persistence::RonFilePersistence;


pub trait Persistence {
    fn save(&self, data: &[Task]) -> Result<()>;
    fn load(&self) -> Result<Vec<Task>>;
}


pub fn get_persistence(location: impl AsRef<Path>) -> Box<dyn Persistence> {
    log::trace!("Creating persistence (RonFilePersistence)");
    Box::new(RonFilePersistence::new(location))
}