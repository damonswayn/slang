//! Runtime-facing API: values, environments, evaluator and builtins.
//!
//! This is a thin logical namespace over the existing modules; it does not
//! introduce new behavior, only groups related runtime pieces together.

pub use crate::env::{Environment, EnvRef};
pub use crate::object::Object;
pub use crate::evaluator::eval;
pub use crate::builtins::get as get_builtin;

use crate::ast::{Program, Statement};
use crate::env::new_env;

/// Summary of running all `test` blocks in a program.
#[derive(Debug, Clone, PartialEq)]
pub struct TestRunSummary {
    pub output: String,
    pub total: usize,
    pub failed: usize,
}

/// Run all `test "name" { ... }` blocks in the given program and return a
/// textual report plus counts. Callers can decide whether to print the
/// output, assert on it (in Rust tests), or ignore it.
pub fn run_tests(program: &Program) -> TestRunSummary {
    // Split program into setup statements and tests.
    let mut setup_statements: Vec<Statement> = Vec::new();
    let mut tests: Vec<(String, Vec<Statement>)> = Vec::new();

    for stmt in &program.statements {
        match stmt {
            Statement::Test(ts) => {
                tests.push((ts.name.clone(), ts.body.statements.clone()));
            }
            other => setup_statements.push(other.clone()),
        }
    }

    // No tests: return a simple message.
    if tests.is_empty() {
        return TestRunSummary {
            output: "No tests found".to_string(),
            total: 0,
            failed: 0,
        };
    }

    use std::fmt::Write as _;

    let mut buf = String::new();
    let mut total = 0usize;
    let mut failed = 0usize;

    for (name, body_stmts) in tests {
        total += 1;

        // Build a synthetic program: setup statements followed by this test body.
        let mut all_statements = setup_statements.clone();
        all_statements.extend(body_stmts.clone());
        let test_program = Program { statements: all_statements };

        let env = new_env();
        let result = eval(&test_program, env);

        match result {
            Object::Error(msg) => {
                failed += 1;
                let _ = writeln!(buf, "FAIL: {} - {}", name, msg);
            }
            _ => {
                let _ = writeln!(buf, "PASS: {}", name);
            }
        }
    }

    let passed = total - failed;
    let _ = writeln!(buf);
    let _ = writeln!(
        buf,
        "Test results: {}/{} passed, {} failed",
        passed, total, failed
    );

    TestRunSummary {
        output: buf,
        total,
        failed,
    }
}

