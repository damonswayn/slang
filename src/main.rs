use std::env::args;
use std::io::{Stdin, Write};
use std::io;
use std::path::Path;
use std::rc::Rc;
use slang::env::{EnvRef, new_env};
use slang::lexer::Lexer;
use slang::parser::Parser;
use slang::runtime::{eval, run_tests, TestRunSummary};

fn main() {
    let env = new_env();
    let stdin = io::stdin();

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        run_repl_mode(Rc::clone(&env), stdin);
    } else {
        if args[1] == "test" {
            run_test_mode(Rc::clone(&env), &args);
        } else {
            run_script_mode(Rc::clone(&env), &args);
        }
    }
}

fn run_script_mode(env: EnvRef, args: &Vec<String>) {
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);
    if !file_path.exists() {
        eprintln!("File not found: {}", file_path_str);
        return;
    }

    let file_content = std::fs::read_to_string(file_path).expect("failed to read file");
    env.borrow_mut()
        .set_module_dir(file_path.parent().map(|p| p.to_path_buf()));
    let lexer = Lexer::new(&file_content);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    println!("{}", eval(&program, env));
    return
}

fn run_test_mode(_base_env: EnvRef, args: &Vec<String>) {
    if args.len() < 3 {
        eprintln!("Usage: slang test <script.sl>");
        return;
    }

    let file_path_str = &args[2];
    let file_path = Path::new(file_path_str);
    if !file_path.exists() {
        eprintln!("File not found: {}", file_path_str);
        return;
    }

    let file_content = std::fs::read_to_string(file_path).expect("failed to read file");
    _base_env
        .borrow_mut()
        .set_module_dir(file_path.parent().map(|p| p.to_path_buf()));
    let lexer = Lexer::new(&file_content);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        eprintln!("Parse errors:");
        for err in parser.errors {
            eprintln!("  {}", err);
        }
        return;
    }

    let summary: TestRunSummary = run_tests(&program);
    println!("{}", summary.output.trim_end());

    if summary.failed > 0 {
        // Non-zero exit code to signal failure when used in CI or scripts.
        std::process::exit(1);
    }
}

fn run_repl_mode(env: EnvRef, stdin: Stdin) {
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

        if input.trim() == "exit;" || input.trim() == "exit"
            || input.trim() == "quit;" || input.trim() == "quit" {
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

        let result = eval(&program, Rc::clone(&env));
        println!("{}", result);
    }
}

fn print_prompt() {
    print!("Slang (ver: {})>> ", env!("CARGO_PKG_VERSION"))
}
