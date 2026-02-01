mod arguments;
mod persistence;
mod tasks;
mod config;
mod error;

use crate::arguments::Commands;
use crate::tasks::NewTask;
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use url::Url;

fn setup_logging(level_filter: LevelFilter) -> Result<()> {
    Ok(TermLogger::init(
        level_filter,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?)
}

#[tokio::main]
async fn main() {
    // Parse the command line arguments.
    let args = arguments::Arguments::parse();

    // Set the logging level to specified, or default to LevelFilter::Warn.
    let logging_level = args.verbosity.unwrap_or(LevelFilter::Warn);
    setup_logging(logging_level).unwrap_or_else(|e| {
        eprintln!("Logger initialization failed: {}", e);
        std::process::exit(1);
    });
    log::debug!("Logging initialised with level: {}", logging_level);

    // Build the path to the config file; The ability to specify an alternative config path has been removed.
    log::trace!("Getting default config path");
    let config_path = config::Config::default_path()
        .expect("Failed to get default path")
        .join("config.toml");
    log::debug!("Config path set to: {}", config_path.to_string_lossy());

    // Default to a file:// url if config isn't read.
    let config = config::Config::from_path(config_path)
        .or_else(|_| {
            log::trace!("No config file found, using default config");
            config::Config::default()
        })
        .unwrap_or_else(|e| {
            log::error!("Failed to get config: {}", e);
            std::process::exit(1);
        });

    // Config here is either read from a file or is default-constructed.

    // Execute the command given in the arguments.
    match args.command {
        // List all tasks.
        Commands::List => {
            log::trace!("Found instruction Commands::List");
            log::debug!("Creating persistence layer");
            let persistence = persistence::get_repository(&config);

            log::info!("Getting tasks");
            let tasks = persistence.get_all().await.unwrap_or_else(|e| {
                log::error!("Failed to get tasks: {}", e);
                std::process::exit(1);
            });

            log::debug!("Listing {} tasks(s):", tasks.len());
            for task in tasks.iter() {
                println!("{}. {}", task.id(), task);
            }

            log::debug!("Done listing tasks, exiting early because no changes were made");
            std::process::exit(0);
        }
        // Add a new task with the given name and optionally a description.
        Commands::Add { name, description } => {
            log::trace!("Found instruction Commands::Add");
            log::debug!("Creating persistence layer");
            let persistence = persistence::get_repository(&config);

            if description.is_none() {
                log::info!("Adding new tasks with no description");
            } else {
                log::info!("Adding new tasks with description");
            }

            let task = NewTask::builder()
                .name(name)
                .maybe_description(description)
                .build();

            persistence.add(task)
                       .await
                       .unwrap_or_else(|e| {
                           log::error!("Failed to add tasks: {}", e);
                           std::process::exit(1);
                       });
        }
        // Remove a task with the given ID.
        Commands::Remove { number } => {
            log::trace!("Found instruction Commands::Remove");
            log::debug!("Creating persistence layer");
            let persistence = persistence::get_repository(&config);

            log::info!("Removing task {}", number);
            persistence.remove(number)
                       .await
                       .unwrap_or_else(|e| {
                           log::error!("Failed to remove tasks: {}", e);
                           std::process::exit(1);
                       });
        }
        // Mark the task with the given ID as completed.
        Commands::Complete { number } => {
            log::trace!("Found instruction Commands::Complete");
            log::debug!("Creating persistence layer");
            let persistence = persistence::get_repository(&config);

            log::info!("Marking task {} as complete", number);
            let mut task = persistence.get_by_id(number)
                                      .await
                                      .unwrap_or_else(|e| {
                                          log::error!("Failed to get tasks: {}", e);
                                          std::process::exit(1);
                                      });

            task.set_completed();

            persistence.update(task)
                       .await
                       .unwrap_or_else(|e| {
                           log::error!("Failed to update tasks: {}", e);
                           std::process::exit(1);
                       })
        }
        // Generate a new configuration file that points to the given storage location.
        Commands::Config { storage: storage_url } => {
            log::trace!("Found instruction Commands::Config");

            if let Some(url) = storage_url {
                log::info!("Constructing a new configuration file");
                let new_storage_url = Url::parse(&url).unwrap_or_else(|e| {
                    log::error!("Failed to parse storage url: {}", e);
                    std::process::exit(1);
                });

                let new_config = config::Config::new_with_url(new_storage_url);

                let config_file_location = config::Config::default_path()
                    .unwrap_or_else(|| {
                        log::error!("Failed to get default path");
                        std::process::exit(1);
                    })
                    .join("config.toml");

                log::debug!("Writing config to {}", config_file_location.to_string_lossy());

                new_config.write_to_file(&config_file_location).unwrap_or_else(|e| {
                    log::error!("Failed to write config: {}", e);
                    std::process::exit(1);
                });

                println!("Config file written to {}", config_file_location.to_string_lossy());
            } else {
                log::info!("No storage URL provided, displaying existing configuration");

                let config_file_location = config::Config::default_path()
                    .unwrap_or_else(|| {
                        log::error!("Failed to get default path");
                        std::process::exit(1);
                    })
                    .join("config.toml");

                let config = config::Config::from_path(config_file_location).unwrap_or_else(|e| {
                    log::error!("Failed to load config: {}", e);
                    std::process::exit(1);
                });
                println!("{}", config);
            }
        }
    }

    // Exit gracefully if we get here.
    log::info!("All done");
}
