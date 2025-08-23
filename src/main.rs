mod task;
mod arguments;


use crate::arguments::Commands;
use crate::task::Task;
use anyhow::Result;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

use ron as parse;

const DEFAULT_LOCATION: &str = "~/.config/tasks.toml";


fn main() -> Result<()> {
    let args = arguments::Arguments::parse();

    let location = args.location.as_deref().unwrap_or(DEFAULT_LOCATION);
    if args.verbose {
        println!("Reading tasks from: {}", location);
    }
    let mut tasks = read_tasks(&location, args.verbose).unwrap_or_default();

    match args.command {
        Commands::List => {
            if args.verbose {
                println!("Listing {} task(s):", tasks.len());
            }
            for (index, task) in tasks.iter().enumerate() {
                println!("{}. {}", index, task);
            }
        }
        Commands::Add { name, description } => {
            if args.verbose {
                println!("Adding new task");
            }
            if let Some(description) = description {
                tasks.push(Task::new(&name, &description));
            } else {
                tasks.push(Task::new_from_name(&name));
            }
        }
        Commands::Remove { number } => {
            if args.verbose {
                println!("Removing task {}", number);
            }
            tasks.remove(number);
        }
        Commands::Complete { number } => {
            if args.verbose {
                println!("Marking task {} as complete", number);
            }
            tasks[number].complete();
        }
    }

    if args.verbose {
        println!("Writing {} task(s) to: {}", tasks.len(), location);
    }
    write_tasks(&location, &tasks, args.verbose)?;

    Ok(())
}


fn read_tasks(location: &dyn AsRef<Path>, verbose: bool) -> Result<Vec<Task>> {
    if !location.as_ref().exists() {
        if verbose {
            println!("No task file found at: {}", location.as_ref().display());
            println!("Returning empty list of tasks.");
        }
        return Ok(Vec::new());
    }

    if verbose {
        println!("Reading tasks from: {}", location.as_ref().display());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(location)?;


    let mut content = String::new();
    let bytes_read = file.read_to_string(&mut content)?;

    if verbose {
        println!("Read {} bytes from file.", bytes_read);
    }

    let tasks: Vec<Task> = parse::from_str(&content)?;

    if verbose {
        println!("Parsed {} task(s) from read bytes", tasks.len());
    }

    Ok(tasks)
}

fn write_tasks(location: &dyn AsRef<Path>, tasks: &Vec<Task>, verbose: bool) -> Result<()> {
    if !location.as_ref().exists() && verbose {
            println!("No task file found at: {}", location.as_ref().display());
            println!("Creating file.");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(location)?;

    let content = parse::to_string(tasks)?;
    if verbose {
        println!("Serialized tasks to {} bytes", content.len());
    }

    Ok(file.write_all(content.as_bytes())?)
}