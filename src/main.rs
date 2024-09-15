use clap::Parser;
use cprs_cli::cli::Cli;

fn main() {
    let args = Cli::parse();
    args.run();
}
