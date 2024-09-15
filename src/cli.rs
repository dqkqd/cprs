use clap::{Parser, Subcommand};

use crate::{history::History, listener, utils::println_to_console};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Listen,
    ListTask { count: usize },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Listen => listener::listen(),
            Commands::ListTask { count } => {
                let tasks = History::get_latest_tasks(count);
                println_to_console(format!("Showing {count} latest tasks:"));
                tasks
                    .iter()
                    .enumerate()
                    .map(|(id, task)| format!(" Id {:>2}: {}", id, task.summary()))
                    .for_each(println_to_console);
            }
        }
        Ok(())
    }
}
