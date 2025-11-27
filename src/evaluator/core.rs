use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::nodes::{ForStatement, PrefixExpression, ReturnStatement};
use crate::ast::{ArrayLiteral, BlockStatement, CallExpression, Expression, FunctionLiteral, Identifier, IfExpression, IndexExpression, InfixExpression, LetStatement, Program, Statement, WhileStatement};
use crate::{builtins, debug_log};
use crate::object::Object;

pub type EnvRef = Rc<RefCell<Environment>>;

/// Simple lexical environment for variables
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed(outer: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(val) = self.store.get(name) {
            Some(val.clone())
        } else if let Some(ref outer) = self.outer {
            outer.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

/// Entry point: evaluate a whole program
pub fn eval(program: &Program, env: EnvRef) -> Object {
    let mut result = Object::Null;

    for stmt in &program.statements {
        result = eval_statement(stmt, Rc::clone(&env));

        if let Object::ReturnValue(val) = result {
            return *val;
        }
    }

    result
}

fn eval_statement(stmt: &Statement, env: EnvRef) -> Object {
    match stmt {
        Statement::Let(ls) => eval_let_statement(ls, Rc::clone(&env)),
        Statement::Return(rs) => eval_return_statement(rs, Rc::clone(&env)),
        Statement::While(ws) => eval_while_statement(ws, Rc::clone(&env)),
        Statement::For(fs) => eval_for_statement(fs, Rc::clone(&env)),
        Statement::Expression(es) => eval_expression(&es.expression, Rc::clone(&env)),
    }
}

fn eval_let_statement(ls: &LetStatement, env: EnvRef) -> Object {
    let val = eval_expression(&ls.value, Rc::clone(&env));
    env.borrow_mut().set(ls.name.value.clone(), val.clone());
    // let itself doesn't produce a useful value
    Object::Null
}

fn eval_expression(expr: &Expression, env: EnvRef) -> Object {
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
        Expression::ArrayLiteral(al) => eval_array_literal(al, env),
        Expression::IndexExpression(ix) => eval_index_expression(ix, env),
    }
}

fn eval_identifier(ident: &Identifier, env: EnvRef) -> Object {
    debug_log!("eval_identifier: looking up '{}'", ident.value);

    let env_borrow = env.borrow();
    if let Some(val) = env_borrow.get(&ident.value) {
        debug_log!("  found in env: {:?}", val);
        return val;
    }

    if let Some(builtin_fn) = builtins::get(&ident.value) {
        debug_log!("  resolved as builtin");
        return Object::Builtin(builtin_fn);
    }

    debug_log!("  not found (returning Null)");
    Object::Null
}

fn eval_infix_expression(infix: &InfixExpression, env: EnvRef) -> Object {
    let left = eval_expression(&infix.left, Rc::clone(&env));
    let right = eval_expression(&infix.right, Rc::clone(&env));

    let op = infix.operator.as_str();

    if op == "=" {
        if let Expression::Identifier(Identifier {value: name}) = &*infix.left {
            let value = eval_expression(&infix.right, Rc::clone(&env));
            env.borrow_mut().set(name.clone(), value.clone());
            return value;
        } else {
            return Object::Null;
        }
    }

    if op == "&&" {
        let left = eval_expression(&infix.left, Rc::clone(&env));

        if !is_truthy(&left) {
            return Object::Boolean(false);
        }

        let right = eval_expression(&infix.right, Rc::clone(&env));
        return Object::Boolean(is_truthy(&right));
    }

    if op == "||" {
        let left = eval_expression(&infix.left, Rc::clone(&env));

        if is_truthy(&left) {
            return Object::Boolean(true);
        }

        let right = eval_expression(&infix.right, Rc::clone(&env));
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
        "/" => Object::Float(left as f64 / right as f64),
        "%" => Object::Float(left as f64 % right as f64),

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

fn eval_block_statement(block: &BlockStatement, env: EnvRef) -> Object {
    let mut result = Object::Null;

    for stmt in &block.statements {
        result = eval_statement(stmt, Rc::clone(&env));

        match result {
            Object::ReturnValue(_) => return result,
            _ => {}
        }
    }

    result
}

fn eval_if_expression(ifexpr: &IfExpression, env: EnvRef) -> Object {
    let condition = eval_expression(&ifexpr.condition, Rc::clone(&env));

    if is_truthy(&condition) {
        eval_block_statement(&ifexpr.consequence, Rc::clone(&env))
    } else if let Some(alt) = &ifexpr.alternative {
        eval_block_statement(alt, Rc::clone(&env))
    } else {
        Object::Null
    }
}

fn eval_prefix_expression(pe: &PrefixExpression, env: EnvRef) -> Object {
    let right = eval_expression(&pe.right, Rc::clone(&env));

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

fn eval_function_literal(fl: &FunctionLiteral, env: EnvRef) -> Object {
    Object::Function {
        params: fl.params.clone(),
        body: fl.body.clone(),
        env
    }
}

fn eval_call_expression(call: &CallExpression, env: EnvRef) -> Object {
    let function = eval_expression(&call.function, Rc::clone(&env));
    let args: Vec<Object> = call
        .arguments
        .iter()
        .map(|arg| eval_expression(arg, Rc::clone(&env)))
        .collect();

    apply_function(function, args)
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { params, body, env } => {
            // // new environment that *encloses* the defining env
            // let mut extended_env = Environment::new_enclosed(env);
            //
            // for (param, arg) in params.iter().zip(args.into_iter()) {
            //     extended_env.set(param.value.clone(), arg);
            // }
            //
            // let result = eval_block_statement(&body, &mut extended_env);
            //
            // match result {
            //     Object::ReturnValue(val) => *val,
            //     _ => result,
            // }
            let extended = Environment::new_enclosed(env);

            {
                let mut inner = extended.borrow_mut();
                for (param, arg) in params.iter().zip(args.into_iter()) {
                    inner.set(param.value.clone(), arg);
                }
            }

            eval_block_statement(&body, extended)
        },
        Object::Builtin(f) => f(args),
        _ => Object::Null, // later: return a proper error object
    }
}

fn eval_return_statement(rs: &ReturnStatement, env: EnvRef) -> Object {
    let val = eval_expression(&rs.return_value, Rc::clone(&env));
    Object::ReturnValue(Box::new(val))
}

fn eval_while_statement(ws: &WhileStatement, env: EnvRef) -> Object {
    let mut result = Object::Null;

    loop {
        let cond = eval_expression(&ws.condition, Rc::clone(&env));
        if !is_truthy(&cond) {
            break;
        }

        result = eval_block_statement(&ws.body, Rc::clone(&env));

        // propagate return out of the loop
        if let Object::ReturnValue(_) = result {
            return result;
        }
    }

    result
}

fn eval_for_statement(fs: &ForStatement, env: EnvRef) -> Object {
    // init
    if let Some(init_stmt) = &fs.init {
        let init_result = eval_statement(init_stmt, Rc::clone(&env));
        if let Object::ReturnValue(_) = init_result {
            return init_result;
        }
    }

    let mut result = Object::Null;

    loop {
        // condition
        if let Some(cond_expr) = &fs.condition {
            let cond = eval_expression(cond_expr, Rc::clone(&env));
            if !is_truthy(&cond) {
                break;
            }
        }

        // body
        result = eval_block_statement(&fs.body, Rc::clone(&env));
        if let Object::ReturnValue(_) = result {
            return result;
        }

        // post
        if let Some(post_stmt) = &fs.post {
            let post_result = eval_statement(post_stmt, Rc::clone(&env));
            if let Object::ReturnValue(_) = post_result {
                return post_result;
            }
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

fn eval_array_literal(al: &ArrayLiteral, env: EnvRef) -> Object {
    let elements = al
        .elements
        .iter()
        .map(|e| eval_expression(e, Rc::clone(&env)))
        .collect::<Vec<_>>();
    Object::Array(elements)
}

fn eval_index_expression(ix: &IndexExpression, env: EnvRef) -> Object {
    let left = eval_expression(&ix.left, Rc::clone(&env));
    let index = eval_expression(&ix.index, Rc::clone(&env));

    match (left, index) {
        (Object::Array(arr), Object::Integer(i)) => eval_array_index(arr, i),
        _ => Object::Null,
    }
}

fn eval_array_index(arr: Vec<Object>, index: i64) -> Object {
    if index < 0 {
        return Object::Null;
    }

    let idx = index as usize;
    if idx >= arr.len() {
        Object::Null
    } else {
        arr[idx].clone()
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

        debug_log!("AST: {} ({} statements)", program, program.statements.len());
        debug_log!("program.statements = {:#?}", program.statements);

        let env = Environment::new();
        eval(&program, env)
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
                Object::Float(f) => assert!((f - expected as f64).abs() < 1e-9, "input: {}", input),
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

    #[test]
    fn test_array_literal() {
        let input = "[1, 2, 3];";

        let obj = eval_input(input);
        match obj {
            Object::Array(elements) => {
                assert_eq!(elements.len(), 3);
                assert_eq!(elements[0], Object::Integer(1));
                assert_eq!(elements[1], Object::Integer(2));
                assert_eq!(elements[2], Object::Integer(3));
            }
            _ => panic!("expected array, got {:?}", obj),
        }
    }

    #[test]
    fn test_array_indexing() {
        let tests = vec![
            ("[1, 2, 3][0];", Some(1)),
            ("[1, 2, 3][1];", Some(2)),
            ("[1, 2, 3][2];", Some(3)),
            ("let a = [1, 2, 3]; a[1];", Some(2)),
            ("[1, 2, 3][3];", None),    // out of range -> null
            ("[1, 2, 3][-1];", None),   // negative -> null
        ];

        for (input, expected) in tests {
            let obj = eval_input(input);
            match (expected, obj) {
                (Some(v), Object::Integer(i)) => assert_eq!(i, v, "input: {}", input),
                (None, Object::Null)          => {},
                _ => panic!("unexpected result for '{}'", input),
            }
        }
    }

    #[test]
    fn test_nested_array_indexing() {
        let input = "let a = [1, [2, 3], 4]; a[1][0];";

        let obj = eval_input(input);
        match obj {
            Object::Integer(i) => assert_eq!(i, 2),
            _ => panic!("expected integer, got {:?}", obj),
        }
    }

    #[test]
    fn test_for_loop_basic() {
        let input = r#"
        let i = 0;
        for (let x = 0; x < 5; x = x + 1) {
            let i = i + 1;
        }
        i;
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(5));
    }

    #[test]
    fn test_for_loop_no_init() {
        let input = r#"
        let i = 0;
        for (; i < 3; ) {
            let i = i + 1;
        }
        i;
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(3));
    }

    #[test]
    fn test_for_loop_with_return() {
        let input = r#"
        fn test() {
            for (let x = 0; ; x = x + 1) {
                if (x == 3) {
                    return x;
                }
            }
        }
        test();
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(3));
    }

    #[test]
    fn test_simple_assignment() {
        let input = r#"
        let x = 1;
        x = x + 2;
        x;
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(3));
    }

    #[test]
    fn test_assignment_returns_value() {
        let input = r#"
        let x = 0;
        let y = (x = 5);
        y;
    "#;

        let obj = eval_input(input);
        assert_eq!(obj, Object::Integer(5));
    }

    #[test]
    fn test_builtin_len() {
        let cases = vec![
            (r#"len("hello");"#, 5),
            (r#"len([1, 2, 3]);"#, 3),
            (r#"len([]);"#, 0),
        ];

        for (input, expected) in cases {
            let obj = eval_input(input);
            match obj {
                Object::Integer(i) => assert_eq!(i, expected, "input: {}", input),
                _ => panic!("expected integer for '{}', got {:?}", input, obj),
            }
        }
    }

    #[test]
    fn test_builtin_first_last_rest_push() {
        let input = r#"
        let a = [1, 2, 3];
        let b = first(a);
        let c = last(a);
        let d = rest(a);
        let e = push(a, 4);
        len(d) + len(e);
    "#;

        let obj = eval_input(input);
        match obj {
            Object::Integer(i) => assert_eq!(i, 2 + 4), // [2,3] len=2; [1,2,3,4] len=4
            _ => panic!("expected integer, got {:?}", obj),
        }
    }
}
