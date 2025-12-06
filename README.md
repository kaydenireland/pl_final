# Language Compiler Project

A minimalistic imperative programming language with lexer, parser, and semantic analyzer.

## Building

```bash
cd /path/to/project
cargo build --release
```

## Running

From the repository root, use the convenient wrapper:

### Print (default test file)
```bash
cargo run print
```

### Parse with verbose output
```bash
cargo run parse lex.txt
```

### Tokenize
```bash
cargo run tokenize lex.txt
```

### Custom files
To run on a custom file, specify the path:
```bash
cargo run parse path/to/myfile.tpl
```

The wrapper automatically resolves paths under `lang/src/` or `lang/`, so you can run:
```bash
cargo run parse myfile.tpl
```
and it will find `lang/src/myfile.tpl` if it exists.

### Alternative (direct invocation)
If you prefer to invoke the compiler directly:
```bash
cd lang
cargo run -- parse src/lex.txt
```

## Features

- **Lexical Analysis**: Hand-coded FSM lexer in `lang/src/lexer.rs`
- **Parsing**: Recursive descent parser (`parser.rs`) with Pratt expression parsing (`pratt_parser.rs`)
- **Semantic Analysis**: Type checking, variable declaration verification, function arity checking

## Test File

The included test file (`lang/src/lex.txt`) demonstrates all language features including error cases for semantic analysis testing.

## Project Structure

```
lang/
  src/
    main.rs          - Entry point, orchestrates CLI and analysis
    cli.rs           - CLI command handler
    lexer.rs         - FSM-based tokenizer
    token.rs         - Token definitions
    parser.rs        - Recursive descent parser
    pratt_parser.rs  - Pratt precedence climbing for expressions
    semantic.rs      - Semantic analysis (type checking, etc.)
    mtree.rs         - Parse tree representation
    lex.txt          - Test input file
  Cargo.toml         - Rust dependencies
src/
  bin/
    print.rs         - Wrapper binary for convenient CLI
Cargo.toml           - Root workspace config
```

## Language Overview

The language supports:
- Functions with parameters and return types
- Integer (`i32`) and boolean (`bool`) types
- Arithmetic: `+`, `-`, `*`, `/`
- Relational: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Control flow: `if`-`else`, `while`, `return`
- Variable declarations: `let x: i32 = 5;`
- Assignments: `x = 10;`
- Function calls: `factorial(n)`
- Print statement: `print result;`

## Example

```tpl
func factorial(n: i32) -> i32 [
    if n < 2 [
        return 1;
    ] else [
        return n * factorial(n - 1);
    ]
]

func main() -> i32 [
    let result: i32 = factorial(5);
    print result;
    return result;
]
```

## Semantic Analysis Output

The compiler reports semantic errors with details:

```
✓ Semantic analysis completed with 3 error(s):
  1. Variable 'undefined_var' not declared
  2. Type mismatch for 'x': expected Int, found Bool
  3. Function 'unknown_func' expects 1 arg but 2 provided
```

For details on coverage of assignment requirements, see `REQUIREMENTS_COVERAGE.md`.
