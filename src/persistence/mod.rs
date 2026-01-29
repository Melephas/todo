mod file_repository;


// PostgreSQL feature
#[cfg(feature = "postgres")]
mod postgres_repository;

#[cfg(feature = "postgres")]
use postgres_repository::get_postgres_repository;


// SQLite feature
#[cfg(feature = "sqlite")]
mod sqlite_repository;

#[cfg(feature = "sqlite")]
use sqlite_repository::get_sqlite_repository;


use crate::config::{Config, StorageFormat};
use crate::persistence::file_repository::FileRepository;
use crate::tasks::{NewTask, Task};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    async fn get_all(&self) -> Result<Vec<Task>>;
    async fn get_by_id(&self, id: i32) -> Result<Task>;
    async fn add(&self, task: NewTask) -> Result<()>;
    async fn remove(&self, id: i32) -> Result<()>;
    async fn update(&self, task: Task) -> Result<()>;
}

pub fn get_repository(config: &Config) -> Box<dyn Repository + Sync> {
    log::debug!("Creating repository");
    match config.storage_format().unwrap_or_else(|err| {
        log::error!("Failed to get storage from config: {}", err);
        std::process::exit(1);
    }) {
        StorageFormat::Postgres => {
            get_postgres_repository(config).unwrap_or_else(|_| panic!("Failed to connect to PostgreSQL database"))
        }
        StorageFormat::Sqlite => {
            get_sqlite_repository(config).unwrap_or_else(|_| panic!("Failed to connect to SQLite database"))
        }
        StorageFormat::LocalStorage => {
            let storage_filepath = config.storage().to_file_path().unwrap_or_else(|_| {
                log::error!("Failed to get storage path from config.");
                std::process::exit(1);
            });

            Box::new(FileRepository::new(storage_filepath).unwrap_or_else(|e| {
                log::error!("Failed to open file repository. ({})", e);
                std::process::exit(1);
            }))
        }
    }
}


#[cfg(not(feature = "postgres"))]
fn get_postgres_repository(_config: &Config) -> Result<Box<dyn Repository + Sync>> {
    Err(anyhow!("Feature \"postgres\" is not enabled, unable to connect to PostgreSQL."))
}


#[cfg(not(feature = "sqlite"))]
fn get_sqlite_repository(_config: &Config) -> Result<Box<dyn Repository + Sync>> {
    Err(anyhow!("Feature \"sqlite\" is not enabled, unable to connect to SQLite."))
}
