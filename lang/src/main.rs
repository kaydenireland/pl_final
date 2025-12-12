mod cli;
mod lexer;
mod parser;
mod pratt_parser;
mod semantic;
mod token;
mod mtree;
mod interpreter;

use clap::Parser;

fn main() {
    // parse CLI
    let args: cli::Cli = cli::Cli::parse();

    // get semantic tree from the command
    cli::handle(args);
    
}






