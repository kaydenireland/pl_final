use clap::{Parser, Subcommand};
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser as LangParser;

// parser returns mtree::MTree, NOT semantic::MTree
use crate::mtree::MTree as ParseTree;

// semantic analysis outputs semantic::MTree
use crate::semantic::{MTree as SemanticTree, from_parse_tree, fold_constants, SymbolTable, analyze};
use crate::interpreter::Interpreter;

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
    Execute {
        filepath: String,
    }
}

pub fn handle(cli: Cli)  {
    match cli.command {
        Command::Print { filepath, numbered } => {
            print_file(filepath, numbered);
        }

        Command::Tokenize { filepath } => {
            tokenize(filepath);
        }

        Command::Parse { filepath } => {
            parse(filepath);
        }

        Command::Execute { filepath } => {
            execute(filepath);
        }
    }
}

fn print_file(path: String, numbered: bool) {
    let contents = fs::read_to_string(path).unwrap();
    if numbered {
        let width = contents.lines().count().to_string().len();
        for (i, line) in contents.lines().enumerate() {
            println!("{:>width$} | {}", i + 1, line, width = width);
        }
    } else {
        println!("{}", contents);
    }
}

fn tokenize(path: String) {
    let contents = fs::read_to_string(path).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.print_tokens();
}

fn parse(path: String) {
    let contents = fs::read_to_string(path).unwrap();

    // correct: parser produces mtree::MTree
    let lexer = Lexer::new(contents);
    let mut parser = LangParser::new(lexer);

    let parse_tree: ParseTree = parser.analyze();

    println!("\n=== Parse Tree ===");
    parse_tree.print();
}

fn execute(path: String) {
    let contents = fs::read_to_string(path).unwrap();

    // correct: parser produces mtree::MTree
    let lexer = Lexer::new(contents);
    let mut parser = LangParser::new(lexer);

    let parse_tree: ParseTree = parser.analyze();

    println!("\n=== Parse Tree ===");
    parse_tree.print();

    // Convert parse tree to semantic tree
    match from_parse_tree(&parse_tree) {
        Ok(mut ast) => {
            println!("\n=== Semantic AST ===\n{:#?}", ast);

            fold_constants(&mut ast);

            // symbol table
            let mut sym_table = SymbolTable::new();

            // run semantic analysis and report how many errors we found
            match analyze(&ast, &mut sym_table) {
                Ok(_) => {
                    println!("\n✓ Semantic analysis completed with 0 error(s).");
                    
                    // If semantic analysis passed, execute the program
                    println!("\n=== Program Execution ===");
                    let mut interp = Interpreter::new();
                    match interp.execute(ast) {
                        Ok(_) => println!("\n✓ Execution completed successfully"),
                        Err(e) => eprintln!("\n✗ Runtime error: {}", e),
                    }
                }
                Err(errors) => {
                    println!("\n✓ Semantic analysis completed with {} error(s):", errors.len());
                    for (i, error) in errors.iter().enumerate() {
                        println!("  {}. {}", i + 1, error);
                    }
                    println!("\n✗ Skipping execution due to semantic errors");
                }
            }
        }
        Err(e) => {
            panic!("Semantic conversion failed: {}", e);
        }
    }

    


}




