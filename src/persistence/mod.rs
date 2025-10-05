mod sql_repository;

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

pub fn get_repository(conn: &str) -> Box<dyn Repository + Sync> {
    Box::new(
        SqlRepository::new(conn)
            .unwrap_or_else(|_| panic!("Failed to connect to database"))
    )
}
