use clap::{Parser, Subcommand};

use crate::listener;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Listen,
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Commands::Listen => listener::listen(),
        }
    }
}
