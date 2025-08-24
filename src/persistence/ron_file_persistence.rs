use crate::persistence::Persistence;
use crate::task::Task;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct RonFilePersistence {
    pub location: PathBuf,
}

impl RonFilePersistence {
    pub fn new(location: impl AsRef<Path>) -> Self {
        log::trace!("Creating new RonFilePersistence");
        Self {
            location: location.as_ref().to_path_buf(),
        }
    }
}

impl Persistence for RonFilePersistence {
    fn save(&self, data: &[Task]) -> Result<()> {
        log::debug!("Opening file: {}", self.location.display());
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.location)
            .unwrap_or_else(|e| {
                log::error!("Failed to open file: {}", e);
                std::process::exit(1);
            });

        log::trace!("Serializing tasks to bytes.");
        let content = ron::to_string(data)?;
        log::debug!("Serialized tasks to {} bytes", content.len());

        log::debug!("Writing tasks to file.");
        Ok(file.write_all(content.as_bytes())?)
    }

    fn load(&self) -> Result<Vec<Task>> {
        log::trace!("Checking if '{}' exists", self.location.display());
        if !self.location.exists() {
            log::debug!("No task file found at: {}", self.location.display());
            log::debug!("Returning empty list of tasks.");
            return Ok(Vec::new());
        }

        log::debug!("Reading tasks from: {}", self.location.display());
        let mut file = OpenOptions::new().read(true).open(&self.location)?;

        log::trace!("Reading file to bytes.");
        let mut content = String::new();
        let bytes_read = file.read_to_string(&mut content)?;
        log::debug!("Read {} bytes from file.", bytes_read);

        let tasks: Vec<Task> = ron::from_str(&content)?;
        log::debug!("Parsed {} task(s) from read bytes", tasks.len());

        Ok(tasks)
    }
}
