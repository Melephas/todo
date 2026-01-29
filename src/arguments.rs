use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser)]
#[command(about, version, author, propagate_version = true)]
pub struct Arguments {
    #[arg(short, long, help = "Sets the log level")]
    pub verbosity: Option<LevelFilter>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Commands {
    #[clap(about = "Create a new configuration or display existing configuration")]
    Config {
        storage: Option<String>,
    },

    #[clap(about = "Lists all tasks")]
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
