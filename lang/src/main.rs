use clap::Parser;

mod cli;
mod token;
mod lexer;
mod parser;
mod pratt_parser;
mod mtree;

fn main() {
    let args: cli::Cli = cli::Cli::parse();
    cli::handle(args);
}