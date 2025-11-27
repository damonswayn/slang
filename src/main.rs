use std::env::args;
use std::io::{Stdin, Write};
use std::io;
use std::path::Path;
use slang::evaluator::{eval, Environment};
use slang::lexer::Lexer;
use slang::parser::Parser;

fn main() {
    let mut env = Environment::new();
    let stdin = io::stdin();

    let args: Vec<String> = args().collect();
    if args.len() < 1 {
        run_repl_mode(&mut env, stdin);
    } else {
        run_script_mode(&mut env, &args);
    }
}

fn run_script_mode(mut env: &mut Environment, args: &Vec<String>) {
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);
    if !file_path.exists() {
        eprintln!("File not found: {}", file_path_str);
        return;
    }

    let file_content = std::fs::read_to_string(file_path).expect("failed to read file");
    let lexer = Lexer::new(&file_content);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{}", eval(&program, &mut env));
    return
}

fn run_repl_mode(mut env: &mut Environment, stdin: Stdin) {
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
