use std::rc::Rc;

use crate::ast::Program;
use crate::env::EnvRef;
use crate::object::Object;

pub mod expr;
mod stmt;

use stmt::eval_statement;

/// Entry point: evaluate a whole program
pub fn eval(program: &Program, env: EnvRef) -> Object {
    let mut result = Object::Null;

    for stmt in &program.statements {
        result = eval_statement(stmt, Rc::clone(&env));

        if let Object::ReturnValue(val) = result {
            return *val;
        }
        if result.is_error() {
            return result;
        }
    }

    result
}

#[cfg(test)]
mod tests;
