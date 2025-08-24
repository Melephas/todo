mod task;
mod arguments;
mod persistence;
mod paths;
mod error;


use std::path::PathBuf;
use crate::arguments::Commands;
use crate::task::Task;
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};
use crate::paths::tasks_file;


fn init_logging(level_filter: LevelFilter) -> Result<()> {
    Ok(TermLogger::init(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?)
}

fn main() {
    let args = arguments::Arguments::parse();

    let verbosity = args.verbosity.unwrap_or(LevelFilter::Warn);

    init_logging(verbosity).unwrap_or_else(|e| {
        eprintln!("Logger initialization failed: {}", e);
        std::process::exit(1);
    });

    let location: PathBuf = args.location.unwrap_or_else(|| {
        tasks_file().unwrap_or_else(|e| {
            log::error!("Failed to get default config path: {}", e);
            std::process::exit(1);
        })
    });

    log::debug!("Creating persistence layer");
    let persistence = persistence::get_persistence(&location);

    log::info!("Reading tasks from: {}", location.display());
    let mut tasks = persistence.load().unwrap_or_else(|e| {
        log::error!("Failed to load tasks: {}", e);
        Vec::new()
    });

    match args.command {
        Commands::List => {
            log::info!("Listing {} task(s):", tasks.len());
            for (index, task) in tasks.iter().enumerate() {
                println!("{}. {}", index, task);
            }

            log::debug!("Done listing tasks, exiting early because no changes were made");
            std::process::exit(0);
        }
        Commands::Add { name, description } => {
            log::info!("Adding new task");
            if let Some(description) = description {
                tasks.push(Task::new(&name, &description));
            } else {
                tasks.push(Task::new_from_name(&name));
            }
        }
        Commands::Remove { number } => {
            log::info!("Removing task {}", number);
            tasks.remove(number);
        }
        Commands::Complete { number } => {
            log::info!("Marking task {} as complete", number);
            tasks[number].complete();
        }
    }

    log::info!("Writing {} task(s) to: {}", tasks.len(), location.display());
    persistence.save(&tasks).unwrap_or_else(|e| {
        log::error!("Failed to save tasks: {}", e);
    });

    log::info!("All done");
}