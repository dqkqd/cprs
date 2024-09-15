use clap::Parser;
use cprs_cli::cli::Cli;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
