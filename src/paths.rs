use crate::error::NoHomeError;
use anyhow::Result;
use std::env::home_dir;
use std::path::PathBuf;

pub fn config_dir() -> Result<PathBuf> {
    let base = home_dir().ok_or(NoHomeError)?;

    let mut config = base.join(".config");
    config.push("todo");

    log::debug!("Config dir path: {:?}", config);

    Ok(config)
}

pub fn tasks_file() -> Result<PathBuf> {
    let config = config_dir()?;
    let tasks = config.join("tasks.ron");

    log::debug!("Tasks file path: {:?}", tasks);

    Ok(tasks)
}
