use std::collections::HashMap;

use crate::ast::{
    Expression,
    Identifier,
    InfixExpression,
    LetStatement,
    Program,
    Statement,
};
use crate::object::Object;

/// Simple lexical environment for variables
#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

/// Entry point: evaluate a whole program
pub fn eval(program: &Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for stmt in &program.statements {
        result = eval_statement(stmt, env);
    }

    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Let(ls) => eval_let_statement(ls, env),
        Statement::Expression(es) => eval_expression(&es.expression, env),
    }
}

fn eval_let_statement(ls: &LetStatement, env: &mut Environment) -> Object {
    let val = eval_expression(&ls.value, env);
    env.set(ls.name.value.clone(), val.clone());
    // let itself doesn't produce a useful value
    Object::Null
}

fn eval_expression(expr: &Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::IntegerLiteral(il) => Object::Integer(il.value),
        Expression::BooleanLiteral(bl) => Object::Boolean(bl.value),
        Expression::Infix(infix) => eval_infix_expression(infix, env),
    }
}

fn eval_identifier(ident: &Identifier, env: &Environment) -> Object {
    if let Some(val) = env.get(&ident.value) {
        val
    } else {
        // for now: unknown identifier becomes null; you could also panic or log an error
        Object::Null
    }
}

fn eval_infix_expression(infix: &InfixExpression, env: &mut Environment) -> Object {
    let left = eval_expression(&infix.left, env);
    let right = eval_expression(&infix.right, env);

    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(&infix.operator, l, r),
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(&infix.operator, l, r),
        _ => Object::Null, // later: type errors, etc.
    }
}

fn eval_integer_infix(op: &str, left: i64, right: i64) -> Object {
    match op {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "%" => Object::Integer(left % right),

        "<" => Object::Boolean(left < right),
        "<=" => Object::Boolean(left <= right),
        ">" => Object::Boolean(left > right),
        ">=" => Object::Boolean(left >= right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

fn eval_boolean_infix(op: &str, left: bool, right: bool) -> Object {
    match op {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval_input(input: &str) -> Object {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        let mut env = Environment::new();
        eval(&program, &mut env)
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            ("1 + 2 * 3;", 7),
            ("(1 + 2) * 3;", 9),
            ("10 - 3 * 2;", 4),
            ("10 / 2 + 3;", 8),
            ("10 % 4;", 2),
        ];

        for (input, expected) in tests {
            let obj = eval_input(input);
            match obj {
                Object::Integer(i) => assert_eq!(i, expected, "input: {}", input),
                _ => panic!("expected integer for '{}', got {:?}", input, obj),
            }
        }
    }

    #[test]
    fn test_let_and_identifier() {
        let input = r#"
            let x = 5 * 10;
            let y = x + 3;
            y;
        "#;
        let obj = eval_input(input);
        match obj {
            Object::Integer(i) => assert_eq!(i, 53),
            _ => panic!("expected integer, got {:?}", obj),
        }
    }

    #[test]
    fn test_boolean_expressions() {
        let tests = vec![
            ("true;", true),
            ("false;", false),
            ("1 < 2;", true),
            ("1 > 2;", false),
            ("1 < 1;", false),
            ("1 <= 1;", true),
            ("1 >= 2;", false),
            ("1 == 1;", true),
            ("1 != 1;", false),
            ("true == true;", true),
            ("true == false;", false),
            ("true != false;", true),
        ];

        for (input, expected) in tests {
            let obj = eval_input(input);
            match obj {
                Object::Boolean(b) => assert_eq!(b, expected, "input: {}", input),
                _ => panic!("expected boolean for '{}', got {:?}", input, obj),
            }
        }
    }
}