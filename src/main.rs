use slang::evaluator::{eval, Environment};
use slang::lexer::Lexer;
use slang::parser::Parser;
use std::io;
use std::io::Write;

fn main() {
    let mut env = Environment::new();
    let stdin = io::stdin();

    loop {
        print_prompt();
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            println!("failed to read input");
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        if input.trim() == "exit;" || input.trim() == "quit;" {
            break;
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            for err in parser.errors {
                println!("{}", err);
            }

            continue;
        }

        let result = eval(&program, &mut env);
        println!("{}", result);
    }
}

fn print_prompt() {
    print!("Slang (ver: {})>> ", env!("CARGO_PKG_VERSION"))
}
