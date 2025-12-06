use clap::{Parser, Subcommand};

use std::fs;

use crate::{
    lexer::Lexer, mtree::MTree as PTree, parser::Parser as lParser, semantic::{MTree as STree, SymbolTable, from_parse_tree, analyze as tree_analyze}
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
    Analyze {
        filepath: String,
    },
}

pub fn handle(cli: Cli) {
    match cli.command {
        Command::Print { filepath, numbered } => print(filepath, numbered),
        Command::Tokenize { filepath } => tokenize(filepath),
        Command::Parse { filepath } => parse(filepath),
        Command::Analyze { filepath } => analyze(filepath),
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

pub fn analyze(path: String) { // semantic analysis
    let lexer = Lexer::new(fs::read_to_string(path).unwrap());
    let mut parser = lParser::new(lexer);
    let mtree: PTree = parser.analyze();
    match from_parse_tree(&mtree) {
        Ok(ast) => {
            println!("\n=== Semantic AST ===\n{:#?}", ast);
            let mut symbol_table = SymbolTable::new();

            match tree_analyze(&ast, &mut symbol_table) {
                Ok(_) => println!("\n✓ Semantic analysis completed with 0 error(s)."),
                Err(errors) => {
                    println!("\n✓ Semantic analysis completed with {} error(s):", errors.len());
                    for (i, error) in errors.iter().enumerate() {
                        println!("  {}. {}", i + 1, error);
                    }
                }
            }
        }
        Err(e) => {
            panic!("Semantic conversion failed: {}", e);
        }
    }

    
}