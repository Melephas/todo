use clap::{Parser, Subcommand};
use log::LevelFilter;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version, author, propagate_version = true)]
pub struct Arguments {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "The location to sync tasks with"
    )]
    pub location: Option<PathBuf>,

    #[arg(short, long, help = "Sets the log level")]
    pub verbosity: Option<LevelFilter>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Lists all the tasks")]
    List,

    #[clap(about = "Adds a new task")]
    Add {
        name: String,
        description: Option<String>,
    },

    #[clap(about = "Removes a task")]
    Remove { number: i32 },

    #[clap(about = "Completes a task")]
    Complete { number: i32 },
}
