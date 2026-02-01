use crate::persistence::Repository;
use crate::tasks::{NewTask, Task};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

pub struct FileRepository {
    storage_path: PathBuf,
    tasks: RwLock<Vec<Task>>,
}

impl FileRepository {
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        log::trace!("Creating new FileRepository with storage path: {}", storage_path.display());

        Self::create_file(&storage_path)?;

        let mut file = OpenOptions::new()
            .read(true)
            .open(&storage_path)?;

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let tasks = ron::de::from_str::<Vec<Task>>(&file_contents)
            .unwrap_or_else(|_| vec![]);

        log::trace!("Found {} task(s) in file", tasks.len());

        Ok(Self {
            storage_path,
            tasks: RwLock::new(tasks),
        })
    }

    async fn write_to_file(&self) -> Result<()> {
        log::trace!("Writing tasks to file");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.storage_path)?;

        let serialised_data = ron::to_string(&self.tasks.read().await.clone())?;
        file.write_all(serialised_data.as_bytes())?;
        Ok(())
    }

    fn create_file(path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        log::trace!("Creating file {}", path.display());

        let file_exists = std::fs::exists(path)?;

        if !file_exists {
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;
        }

        Ok(())
    }
}

#[async_trait]
impl Repository for FileRepository {
    async fn get_all(&self) -> Result<Vec<Task>> {
        log::trace!("Getting all tasks");
        let tasks = self.tasks.read().await;
        Ok(tasks.clone())
    }

    async fn get_by_id(&self, id: i32) -> Result<Task> {
        log::trace!("Getting task with id {}", id);
        let tasks = self.tasks.read().await;
        Ok(tasks.iter()
                .find(|task| task.id() == id)
                .ok_or_else(|| anyhow!("No task with id {} found", id))?
            .clone()
        )
    }

    async fn add(&self, task: NewTask) -> Result<()> {
        log::trace!("Adding new task");
        {
            let mut tasks = self.tasks.write().await;
            let new_id = tasks.iter()
                              .map(|a| a.id())
                              .max()
                              .map(|max| max + 1)
                              .unwrap_or(1);

            let new_task = Task::builder()
                .id(new_id)
                .name(task.name().clone())
                .maybe_description(task.description().cloned())
                .completed(false)
                .build();

            tasks.push(new_task);
        }
        self.write_to_file().await?;
        Ok(())
    }

    async fn remove(&self, id: i32) -> Result<()> {
        log::trace!("Removing task with id {}", id);
        {
            let mut tasks = self.tasks.write().await;
            tasks.retain(|task| task.id() != id);
        }
        self.write_to_file().await?;
        Ok(())
    }

    async fn update(&self, task: Task) -> Result<()> {
        log::trace!("Updating task with id {}", task.id());
        {
            let mut tasks = self.tasks.write().await;
            let index = tasks.iter()
                             .position(|a| a.id() == task.id())
                             .ok_or_else(|| anyhow!("No task with id {} found", task.id()))?;
            tasks[index] = task;
        }
        self.write_to_file().await?;
        Ok(())
    }
}
