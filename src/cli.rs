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
    Init,
    Listen,
    ListTask { count: usize },
    Cd { task_id: usize },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Init => {
                let function = r#"
cprs() {
  if [[ "$#" -eq 2 && "$1" == "cd" ]]
  then
    result=$(cprs_cli "$@")
    echo "Change directory to $result"
    cd $result
  else
    echo "??"
    cprs_cli "$@"
  fi
}
                "#;
                println_to_console(function);
            }
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
            Commands::Cd { task_id } => {
                let task = History::get_task(task_id)?;
                println_to_console(task.task_folder()?.display());
            }
        }
        Ok(())
    }
}
