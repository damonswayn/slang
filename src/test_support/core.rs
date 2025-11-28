//! Test-only support helpers shared across modules.
//!
//! This module is only compiled in test builds (`#[cfg(test)]` in `lib.rs`).

use crate::debug_log;
use crate::env::new_env;
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

/// Lex, parse and evaluate a snippet of Slang code, returning the resulting `Object`.
pub fn eval_input(input: &str) -> Object {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    debug_log!("AST: {} ({} statements)", program, program.statements.len());
    debug_log!("program.statements = {:#?}", program.statements);

    let env = new_env();
    eval(&program, env)
}

/// Panic if the parser has accumulated any errors.
pub fn check_errors(p: &Parser) {
    if !p.errors.is_empty() {
        panic!("parser had errors: {:?}", p.errors);
    }
}


