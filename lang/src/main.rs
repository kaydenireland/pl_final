use clap::Parser;

mod cli;

fn main() {
    let args: cli::Cli = cli::Cli::parse();
    cli::handle(args);
}