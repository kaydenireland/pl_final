use clap::{Parser, Subcommand};

use std::fs;

use crate::{
    lexer::Lexer,
    parser::Parser as lParser
};


#[derive(Parser)]
#[command(name = "lang", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    Print {
        filepath: String,
        #[arg(short, long)]
        numbered: bool,
    },
    Tokenize {
        filepath: String,
    },
    Parse {
        filepath: String,
    },
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Tokenize { filepath } => tokenize(filepath),
        Command::Parse { filepath } => parse(filepath),
    }
}

pub fn print(path: String, numbered: bool) {
    let contents = std::fs::read_to_string(path).unwrap();

    if numbered {
        let total = contents.lines().count();
        let width = total.to_string().len();

        for (i, line) in contents.lines().enumerate() {
            let num = format!("{:>width$}", i + 1, width = width);
            println!("{} | {}", num, line);
        }
    } else {
        println!("{}", contents);
    }
}

pub fn tokenize(path: String) {
    let contents = fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}

pub fn parse(path: String) {
    let lexer = Lexer::new(fs::read_to_string(path).unwrap());
    let mut parser = lParser::new(lexer);
    let mtree = parser.analyze();

    println!("\nMTree:");
    mtree.print();
}
