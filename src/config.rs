use crate::error::NoConfigError;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use toml as ser_fmt;
use url::Url;

pub enum StorageFormat {
    Postgres,
    LocalStorage,
}

#[derive(Debug, Hash, Deserialize, Serialize, Clone)]
pub struct Config {
    storage: Url,
}

impl Config {
    pub fn default() -> Result<Self> {
        let mut path = Self::default_path().ok_or_else(|| {
            anyhow!("Failed to get default config path")
        })?;
        path.push("default.todo.ron");

        let storage_url = format!("file://{}", path.to_string_lossy());

        Ok(Config {
            storage: Url::parse(&storage_url)?,
        })
    }
    pub fn default_path() -> Option<PathBuf> {
        let home_path = std::env::home_dir()?;
        let config_path = home_path.join(".config");
        let todo_path = config_path.join("todo");

        Some(todo_path)
    }

    pub fn from_file(mut file: File) -> Result<Self> {
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        let file_contents = file_contents.trim_end();
        log::trace!("Read config file contents: {}", file_contents);
        let config = ser_fmt::from_str(file_contents)?;
        Ok(config)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let file = OpenOptions::new()
                .read(true)
                .open(path)?;
            return Self::from_file(file);
        }
        Err(NoConfigError(path.into()).into())
    }

    pub fn new_with_url(storage: Url) -> Self {
        Self {
            storage,
        }
    }

    #[allow(dead_code)]
    pub fn new(storage: &str) -> Result<Self> {
        Ok(Self {
            storage: Url::parse(storage)?,
        })
    }

    pub fn write_to_file(&self, location: impl AsRef<Path>) -> Result<()> {
        let path = location.as_ref();

        let serialised_config = ser_fmt::to_string(&self)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(serialised_config.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn storage_format(&self) -> Result<StorageFormat> {
        match self.storage.scheme() {
            "postgresql" => Ok(StorageFormat::Postgres),
            "file" => Ok(StorageFormat::LocalStorage),
            _ => Err(anyhow!("Unsupported format")),
        }
    }

    pub fn storage(&self) -> &Url {
        &self.storage
    }
}

mod test {
    #[test]
    fn output_serialised_config() {
        let config = super::Config::new("file:///home/user/.config/todo/todos").unwrap();
        let serialized_config = super::ser_fmt::to_string(&config).unwrap();
        println!("{}", serialized_config);
    }
}
