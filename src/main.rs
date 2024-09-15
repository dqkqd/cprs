use clap::Parser;
use cprs::cli::Cli;

fn main() {
    let args = Cli::parse();
    args.run();
}
