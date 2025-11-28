use crate::object::Object;
use crate::test_support::eval_input;

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
            (None, Object::Null) => {}
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
fn test_object_field_assignment_simple() {
    let input = r#"
        let p = { x: 1 };
        p.x = 10;
        p.x;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 10),
        _ => panic!("expected integer, got {:?}", obj),
    }
}

#[test]
fn test_object_field_assignment_creates_new_field() {
    let input = r#"
        let p = { };
        p.x = 42;
        p.x;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 42),
        _ => panic!("expected integer, got {:?}", obj),
    }
}

#[test]
fn test_nested_object_field_assignment() {
    let input = r#"
        let p = { inner: { x: 1 } };
        p.inner.x = 99;
        p.inner.x;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 99),
        _ => panic!("expected integer, got {:?}", obj),
    }
}

#[test]
fn test_method_call_with_this() {
    let input = r#"
        let p = {
            x: 5,
            add_to_x: fn(a) { this.x + a; },
        };

        p.add_to_x(10);
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 15),
        _ => panic!("expected integer, got {:?}", obj),
    }
}

#[test]
fn test_object_literal_and_property_access() {
    let input = r#"
        let p = { x: 1, y: 2 };
        p.x + p.y;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 3),
        _ => panic!("expected integer, got {:?}", obj),
    }
}

#[test]
fn test_nested_object_property_access() {
    let input = r#"
        let p = { x: 1, inner: { y: 2 } };
        p.inner.y;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 2),
        _ => panic!("expected integer, got {:?}", obj),
    }
}

#[test]
fn test_missing_property_returns_null() {
    let input = r#"
        let p = { x: 1 };
        p.y;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Null => {}
        _ => panic!("expected null for missing property, got {:?}", obj),
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

#[test]
fn test_function_statement() {
    let input = r#"
        function fact(n) {
            if (n == 0) {
                1;
            } else {
                n * fact(n - 1);
            }
        }

        fact(5);
        "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(120));
}


