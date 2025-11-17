mod sql_repository;
mod file_repository;

use crate::config::{Config, StorageFormat};
use crate::persistence::file_repository::FileRepository;
use crate::persistence::sql_repository::SqlRepository;
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
            Box::new(
                SqlRepository::new(config.storage())
                    .unwrap_or_else(|_| panic!("Failed to connect to database"))
            )
        }
        StorageFormat::LocalStorage => {
            let storage_filepath = config.storage().to_file_path().unwrap_or_else(|_| {
                log::error!("Failed to get storage path from config.");
                std::process::exit(1);
            });

            Box::new(
                FileRepository::new(storage_filepath).unwrap_or_else(|e| {
                    log::error!("Failed to open file repository. ({})", e);
                    std::process::exit(1);
                })
            )
        }
    }
}
