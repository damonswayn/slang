use std::collections::HashMap;

use crate::ast::nodes::{PrefixExpression, ReturnStatement};
use crate::ast::{
    BlockStatement, CallExpression, Expression, FunctionLiteral, Identifier, IfExpression,
    InfixExpression, LetStatement, Program, Statement, WhileStatement,
};
use crate::object::Object;

/// Simple lexical environment for variables
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Environment) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            Some(obj.clone())
        } else if let Some(ref outer) = self.outer {
            outer.get(name)
        } else {
            None
        }
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

        if let Object::ReturnValue(val) = result {
            return *val;
        }
    }

    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Let(ls) => eval_let_statement(ls, env),
        Statement::Return(rs) => eval_return_statement(rs, env),
        Statement::While(ws) => eval_while_statement(ws, env),
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
        Expression::FloatLiteral(fl) => Object::Float(fl.value),
        Expression::BooleanLiteral(bl) => Object::Boolean(bl.value),
        Expression::StringLiteral(sl) => Object::String(sl.value.clone()),
        Expression::Infix(infix) => eval_infix_expression(infix, env),
        Expression::If(ifexpr) => eval_if_expression(ifexpr, env),
        Expression::Prefix(p) => eval_prefix_expression(p, env),
        Expression::FunctionLiteral(fl) => eval_function_literal(fl, env),
        Expression::CallExpression(call) => eval_call_expression(call, env),
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

    let op = infix.operator.as_str();

    if op == "&&" {
        let left = eval_expression(&infix.left, env);

        if !is_truthy(&left) {
            return Object::Boolean(false);
        }

        let right = eval_expression(&infix.right, env);
        return Object::Boolean(is_truthy(&right));
    }

    if op == "||" {
        let left = eval_expression(&infix.left, env);

        if is_truthy(&left) {
            return Object::Boolean(true);
        }

        let right = eval_expression(&infix.right, env);
        return Object::Boolean(is_truthy(&right));
    }

    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(&infix.operator, l, r),
        (Object::Float(l), Object::Float(r)) => eval_float_infix(&infix.operator, l, r),

        // mixed numeric types are coerced to float, so we can use the same logic as for integers
        (Object::Integer(l), Object::Float(r)) => eval_float_infix(&infix.operator, l as f64, r),
        (Object::Float(l), Object::Integer(r)) => eval_float_infix(&infix.operator, l, r as f64),

        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(&infix.operator, l, r),
        (Object::String(l), Object::String(r)) => eval_string_infix(&infix.operator, &l, &r),
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

fn eval_float_infix(op: &str, left: f64, right: f64) -> Object {
    match op {
        "+" => Object::Float(left + right),
        "-" => Object::Float(left - right),
        "*" => Object::Float(left * right),
        "/" => Object::Float(left / right),
        "%" => Object::Float(left % right),

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

fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for stmt in &block.statements {
        result = eval_statement(stmt, env);

        match result {
            Object::ReturnValue(_) => return result,
            _ => {}
        }
    }

    result
}

fn eval_if_expression(ifexpr: &IfExpression, env: &mut Environment) -> Object {
    let condition = eval_expression(&ifexpr.condition, env);

    if is_truthy(&condition) {
        eval_block_statement(&ifexpr.consequence, env)
    } else if let Some(alt) = &ifexpr.alternative {
        eval_block_statement(alt, env)
    } else {
        Object::Null
    }
}

fn eval_prefix_expression(pe: &PrefixExpression, env: &mut Environment) -> Object {
    let right = eval_expression(&pe.right, env);

    match pe.operator.as_str() {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_prefix(right), // already existing
        _ => Object::Null,
    }
}

fn eval_bang_operator(obj: Object) -> Object {
    match obj {
        Object::Boolean(b) => Object::Boolean(!b),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix(obj: Object) -> Object {
    match obj {
        Object::Integer(i) => Object::Integer(-i),
        Object::Float(f) => Object::Float(-f),
        _ => Object::Null, // or some error type later
    }
}

fn eval_function_literal(fl: &FunctionLiteral, env: &Environment) -> Object {
    Object::Function {
        params: fl.params.clone(),
        body: fl.body.clone(),
        env: env.clone(), // capture defining environment (simple closures)
    }
}

fn eval_call_expression(call: &CallExpression, env: &mut Environment) -> Object {
    let function = eval_expression(&call.function, env);
    let args: Vec<Object> = call
        .arguments
        .iter()
        .map(|arg| eval_expression(arg, env))
        .collect();

    apply_function(function, args)
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { params, body, env } => {
            // new environment that *encloses* the defining env
            let mut extended_env = Environment::new_enclosed(env);

            for (param, arg) in params.iter().zip(args.into_iter()) {
                extended_env.set(param.value.clone(), arg);
            }

            let result = eval_block_statement(&body, &mut extended_env);

            match result {
                Object::ReturnValue(val) => *val,
                _ => result,
            }
        }
        _ => Object::Null, // later: return a proper error object
    }
}

fn eval_return_statement(rs: &ReturnStatement, env: &mut Environment) -> Object {
    let val = eval_expression(&rs.return_value, env);
    Object::ReturnValue(Box::new(val))
}

fn eval_while_statement(ws: &WhileStatement, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    loop {
        let cond = eval_expression(&ws.condition, env);
        if !is_truthy(&cond) {
            break;
        }

        result = eval_block_statement(&ws.body, env);

        // propagate return out of the loop
        if let Object::ReturnValue(_) = result {
            return result;
        }
    }

    result
}

fn eval_string_infix(op: &str, left: &str, right: &str) -> Object {
    match op {
        "+"  => {
            let mut s = String::with_capacity(left.len() + right.len());
            s.push_str(left);
            s.push_str(right);
            Object::String(s)
        }
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _    => Object::Null,
    }
}



fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(false) => false,
        Object::Null => false,
        _ => true, // everything else is truthy
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

    #[test]
    fn test_float_arithmetic_and_comparisons() {
        let tests = vec![
            ("1.5 + 2.25;", 3.75),
            ("10.0 - 3.5;", 6.5),
            ("2.0 * 4.5;", 9.0),
            ("9.0 / 4.5;", 2.0),
            ("1.0 < 2.0;", 1.0), // use 1.0 for true if you want? or test as boolean separately
        ];

        for (input, expected) in &tests[0..4] {
            let obj = eval_input(input);
            match obj {
                Object::Float(x) => assert!((x - expected).abs() < 1e-9, "input: {}", input),
                _ => panic!("expected float for '{}', got {:?}", input, obj),
            }
        }

        // comparison example
        let obj = eval_input("1.5 < 2.0;");
        match obj {
            Object::Boolean(b) => assert!(b),
            _ => panic!("expected boolean, got {:?}", obj),
        }
    }

    #[test]
    fn test_if_expressions() {
        let cases = vec![
            ("if (true) { 10; }", Some(10)),
            ("if (false) { 10; }", None),
            ("if (1 < 2) { 10; }", Some(10)),
            ("if (1 > 2) { 10; }", None),
            ("if (1 > 2) { 10; } else { 20; }", Some(20)),
            ("if (false) { 10; } else { 30; }", Some(30)),
            ("if (false && true) { 10; } else { 30; }", Some(30)),
            ("if (true && true) { 10; } else { 30; }", Some(10)),
        ];

        for (input, expected) in cases {
            let obj = eval_input(input);
            match (expected, &obj) {
                (Some(v), Object::Integer(i)) => assert_eq!(*i, v, "input: {}", input),
                (None, Object::Null) => {}
                _ => panic!("unexpected result for '{}': {:?}", input, obj),
            }
        }
    }

    #[test]
    fn test_unary_minus_and_not() {
        let tests = vec![
            ("-5;", Object::Integer(-5)),
            ("-10 + 5;", Object::Integer(-5)), // (-10) + 5
            ("-(1 + 2);", Object::Integer(-3)),
            ("-1.5;", Object::Float(-1.5)),
            ("!-true;", Object::Boolean(true)),
        ];

        for (input, expected) in tests {
            let obj = eval_input(input);
            assert_eq!(obj, expected, "input: {}", input);
        }
    }

    #[test]
    fn test_unary_minus_precedence() {
        let obj = eval_input("-1 * 2;");
        match obj {
            Object::Integer(i) => assert_eq!(i, -2),
            _ => panic!("expected integer, got {:?}", obj),
        }
    }

    #[test]
    fn test_logical_operators() {
        let tests = vec![
            ("!true;", false),
            ("!false;", true),
            ("!!true;", true),
            ("!!false;", false),
            ("true && true;", true),
            ("true && false;", false),
            ("false && true;", false),
            ("true || false;", true),
            ("false || false;", false),
            ("false || true;", true),
            ("1 < 2 && 2 < 3;", true),
            ("1 < 2 && 2 > 3;", false),
            ("1 > 2 || 2 > 3;", false),
            ("false || 1 < 2;", true),
        ];

        for (input, expected) in tests {
            let obj = eval_input(input);
            match obj {
                Object::Boolean(b) => assert_eq!(b, expected, "input: {}", input),
                _ => panic!("expected boolean for '{}', got {:?}", input, obj),
            }
        }
    }

    #[test]
    fn test_function_application() {
        let input = r#"
        let identity = function(x) { x; };
        identity(5);
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(5));
    }

    #[test]
    fn test_function_with_two_params() {
        let input = r#"
        let add = function(a, b) { a + b; };
        add(2, 3);
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(5));
    }

    #[test]
    fn test_closure_capture() {
        let input = r#"
        let a = 10;
        let f = function(x) { x + a; };
        f(5);
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(15));
    }

    #[test]
    fn test_simple_return() {
        let input = r#"
        let f = function() { return 10; };
        f();
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(10));
    }

    #[test]
    fn test_return_last_expression_fallback() {
        let input = r#"
        let f = function() { 10; 20; };
        f();
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(20));
    }

    #[test]
    fn test_return_early_exit() {
        let input = r#"
        let f = function() {
            let x = 1;
            if (true) {
                return 10;
            }
            x + 100; // should not run
        };
        f();
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(10));
    }

    #[test]
    fn test_while_loop_basic() {
        let input = r#"
        let x = 0;
        while (x < 5) {
            let x = x + 1;
        }
        x;
    "#;

        let obj = eval_input(input);
        match obj {
            Object::Integer(i) => assert_eq!(i, 5),
            _ => panic!("expected integer, got {:?}", obj),
        }
    }

    #[test]
    fn test_while_with_return() {
        let input = r#"
        let f = fn() {
            let x = 0;
            while (x < 5) {
                if (x == 3) {
                    return x;
                }
                let x = x + 1;
            }
            99;
        };
        f();
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(3));
    }

    #[test]
    fn test_string_literal() {
        let input = r#""hello world";"#;

        let obj = eval_input(input);
        match obj {
            Object::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("expected string, got {:?}", obj),
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = r#""hello" + " " + "world";"#;

        let obj = eval_input(input);
        match obj {
            Object::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("expected string, got {:?}", obj),
        }
    }

    #[test]
    fn test_string_equality() {
        let tests = vec![
            (r#""a" == "a";"#, true),
            (r#""a" == "b";"#, false),
            (r#""foo" != "bar";"#, true),
        ];

        for (input, expected) in tests {
            let obj = eval_input(input);
            match obj {
                Object::Boolean(b) => assert_eq!(b, expected, "input: {}", input),
                _ => panic!("expected boolean, got {:?}", obj),
            }
        }
    }
}
