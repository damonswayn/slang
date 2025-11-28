pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod object;
pub mod env;
pub mod evaluator;
pub mod runtime;
pub mod builtins;
pub mod debug;

#[cfg(test)]
pub mod test_support;

// Public API re-exports for ergonomic crate usage
pub use token::{Token, TokenType, lookup_ident};
pub use lexer::Lexer;
pub use parser::Parser;
pub use ast::{Program, Statement, Expression};
pub use object::Object;
pub use env::{Environment, EnvRef};
pub use evaluator::eval;
pub use builtins::get as get_builtin;

/// Convenient prelude for common interpreter types and functions.
pub mod prelude {
    pub use crate::lexer::Lexer;
    pub use crate::parser::Parser;
    pub use crate::ast::{Program, Statement, Expression};
    pub use crate::object::Object;
    pub use crate::env::{Environment, EnvRef};
    pub use crate::evaluator::eval;
}