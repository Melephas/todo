mod arguments;
mod persistence;
mod tasks;

use crate::arguments::Commands;
use crate::tasks::NewTask;
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn init_logging(level_filter: LevelFilter) -> Result<()> {
    Ok(TermLogger::init(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args = arguments::Arguments::parse();

    let verbosity = args.verbosity.unwrap_or(LevelFilter::Warn);

    init_logging(verbosity).unwrap_or_else(|e| {
        eprintln!("Logger initialization failed: {}", e);
        std::process::exit(1);
    });

    log::debug!("Creating persistence layer");
    let persistence = persistence::get_repository(
        dotenvy::var("DATABASE_URL").unwrap_or_else(|_| {
            log::error!("Failed to get database URL from environment");
            std::process::exit(1);
        }).as_str()
    );

    match args.command {
        Commands::List => {
            log::info!("Getting tasks");
            let tasks = persistence.get_all().await.unwrap_or_else(|e| {
                log::error!("Failed to get tasks: {}", e);
                std::process::exit(1);
            });

            log::debug!("Listing {} tasks(s):", tasks.len());
            for task in tasks.iter() {
                println!("{}. {}", task.id, task);
            }

            log::debug!("Done listing tasks, exiting early because no changes were made");
            std::process::exit(0);
        }
        Commands::Add { name, description } => {
            if description.is_none() {
                log::info!("Adding new tasks with no description");
            } else {
                log::info!("Adding new tasks with description");
            }

            let task = NewTask {
                name,
                description,
            };

            persistence.add(task)
                       .await
                       .unwrap_or_else(|e| {
                           log::error!("Failed to add tasks: {}", e);
                           std::process::exit(1);
                       });
        }
        Commands::Remove { number } => {
            log::info!("Removing tasks {}", number);
            persistence.remove(number)
                       .await
                       .unwrap_or_else(|e| {
                           log::error!("Failed to remove tasks: {}", e);
                           std::process::exit(1);
                       });
        }
        Commands::Complete { number } => {
            log::info!("Marking tasks {} as complete", number);
            let mut task = persistence.get_by_id(number)
                                      .await
                                      .unwrap_or_else(|e| {
                                          log::error!("Failed to get tasks: {}", e);
                                          std::process::exit(1);
                                      });

            task.complete();

            persistence.update(task)
                       .await
                       .unwrap_or_else(|e| {
                           log::error!("Failed to update tasks: {}", e);
                           std::process::exit(1);
                       })
        }
    }

    log::info!("All done");
}
