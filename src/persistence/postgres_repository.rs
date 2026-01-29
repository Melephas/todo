use crate::config::Config;
use crate::persistence::Repository;
use crate::tasks::{NewTask, Task};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use url::Url;

pub struct PostgresRepository {
    connection_pool: PgPool,
}

impl PostgresRepository {
    pub fn new(connection_url: &Url) -> Result<Self> {
        log::trace!("Creating new PostgresRepository with URL: {}", connection_url);
        Ok(PostgresRepository {
            connection_pool: PgPool::connect_lazy((&connection_url).as_ref())?,
        })
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn get_all(&self) -> Result<Vec<Task>> {
        log::trace!("Getting all tasks");
        let rows = sqlx::query_as::<_, Task>("select * from tasks order by id")
            .fetch_all(&self.connection_pool)
            .await?;

        Ok(rows)
    }

    async fn get_by_id(&self, id: i32) -> Result<Task> {
        log::trace!("Getting task with ID {}", id);
        let task = sqlx::query_as::<_, Task>("select * from tasks where id = $1")
            .bind(id)
            .fetch_optional(&self.connection_pool)
            .await?;

        task.ok_or_else(|| anyhow::anyhow!("No tasks with id {} found", id))
    }

    async fn add(&self, task: NewTask) -> Result<()> {
        log::trace!("Adding a new task");
        sqlx::query("insert into tasks (name, description) values ($1, $2)")
            .bind(task.name)
            .bind(task.description)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }

    async fn remove(&self, id: i32) -> Result<()> {
        log::trace!("Removing task with ID {}", id);
        sqlx::query("delete from tasks where id = $1")
            .bind(id)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }

    async fn update(&self, task: Task) -> Result<()> {
        log::trace!("Updating task with ID {}", task.id);
        sqlx::query("update tasks set name = $1, description = $2, completed = $3 where id = $4")
            .bind(task.name)
            .bind(task.description)
            .bind(task.completed)
            .bind(task.id)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }
}

pub fn get_postgres_repository(config: &Config) -> Result<Box<dyn Repository + Sync>> {
    Ok(Box::from(PostgresRepository::new(config.storage())?))
}
