use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, version, author, propagate_version = true)]
pub struct Arguments {
    #[arg(short, long, value_name = "FILE", help = "The location to sync tasks with")]
    pub location: Option<String>,

    #[arg(short, long, help = "Prints more updates about the working of the program")]
    pub verbose: bool,

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
    Remove { number: usize },

    #[clap(about = "Completes a task")]
    Complete { number: usize },
}