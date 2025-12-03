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
fn test_increment_decrement_on_variables() {
    let cases = vec![
        // prefix increment: ++x yields new value and updates x
        (r#"
            let x = 1;
            ++x;
        "#, Object::Integer(2)),
        // postfix increment: x++ yields old value in expression but updates x
        (r#"
            let x = 1;
            let y = x++;
            y + x;
        "#, Object::Integer(3)),
        // prefix decrement
        (r#"
            let x = 3;
            let y = --x;
            y + x;
        "#, Object::Integer(4)),
        // postfix decrement
        (r#"
            let x = 3;
            let y = x--;
            y + x;
        "#, Object::Integer(5)),
    ];

    for (input, expected) in cases {
        let obj = eval_input(input);
        assert_eq!(obj, expected, "input: {}", input);
    }
}

#[test]
fn test_increment_decrement_on_object_properties() {
    let cases = vec![
        // postfix on property
        (r#"
            let p = { x: 1 };
            let y = p.x++;
            y + p.x;
        "#, Object::Integer(3)),
        // prefix on property
        (r#"
            let p = { x: 1 };
            let y = ++p.x;
            y + p.x;
        "#, Object::Integer(4)),
    ];

    for (input, expected) in cases {
        let obj = eval_input(input);
        assert_eq!(obj, expected, "input: {}", input);
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
fn test_higher_order_function_returning_function() {
    let input = r#"
        let makeAdder = fn(x) {
            function(y) { x + y; }; // inner fn closes over x and is returned
        };

        let addTwo = makeAdder(2);
        let addTen = makeAdder(10);

        addTwo(3) + addTen(7);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(5 + 17));
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
fn test_regex_builtins() {
    // isMatch
    let input = r#"
        let t1 = Regex::isMatch("hello123", "[a-z]+[0-9]+");
        let t2 = Regex::isMatch("hello", "[0-9]+");

        let m1 = Regex::find("abc123xyz", "\d+");
        let m2 = Regex::find("no-digits-here", "\d+");

        let r = Regex::replace("foo 123 bar 456", "\d+", "X");

        let c1 = Regex::match("abc123", "([a-z]+)(\d+)");
        let c2 = Regex::match("no-digits", "(\d+)");

        [t1, t2, m1, m2, r, c1, c2];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 7);

            assert_eq!(vals[0], Object::Boolean(true));  // matches
            assert_eq!(vals[1], Object::Boolean(false)); // does not match

            match &vals[2] {
                Object::OptionSome(inner) => match &**inner {
                    Object::String(s) => assert_eq!(s, "123"),
                    other => panic!("expected inner string \"123\" for m1, got {:?}", other),
                },
                other => panic!("expected Option::Some(\"123\") for m1, got {:?}", other),
            }

            assert_eq!(vals[3], Object::OptionNone); // no match

            match &vals[4] {
                Object::String(s) => assert_eq!(s, "foo X bar X"),
                other => panic!("expected replaced string, got {:?}", other),
            }

            // c1: regexMatch with capture groups
            match &vals[5] {
                Object::OptionSome(inner) => match &**inner {
                    Object::Array(groups) => {
                        assert_eq!(groups.len(), 3);
                        assert_eq!(groups[0], Object::String("abc123".to_string())); // full match
                        assert_eq!(groups[1], Object::String("abc".to_string()));    // group 1
                        assert_eq!(groups[2], Object::String("123".to_string()));    // group 2
                    }
                    other => panic!("expected array of groups for c1, got {:?}", other),
                },
                other => panic!("expected Option::Some([...]) for c1, got {:?}", other),
            }

            // c2: no match -> None
            assert_eq!(vals[6], Object::OptionNone);
        }
        other => panic!("expected array from regex builtins test, got {:?}", other),
    }
}

#[test]
fn test_option_constructors() {
    // Some
    let some = eval_input("Option::Some(5);");
    match some {
        Object::OptionSome(inner) => assert_eq!(*inner, Object::Integer(5)),
        other => panic!("expected Option::Some(5), got {:?}", other),
    }

    // None
    let none = eval_input("Option::None();");
    assert_eq!(none, Object::OptionNone);
}

#[test]
fn test_result_constructors() {
    // Ok
    let ok = eval_input("Result::Ok(42);");
    match ok {
        Object::ResultOk(inner) => assert_eq!(*inner, Object::Integer(42)),
        other => panic!("expected Result::Ok(42), got {:?}", other),
    }

    // Err with string
    let err = eval_input(r#"Result::Err("oops");"#);
    match err {
        Object::ResultErr(inner) => match *inner {
            Object::String(s) => assert_eq!(s, "oops"),
            v => panic!("expected inner string \"oops\", got {:?}", v),
        },
        other => panic!("expected Result::Err(\"oops\"), got {:?}", other),
    }
}

#[test]
fn test_option_helpers() {
    let input = r#"
        let some = Option::Some(5);
        let none = Option::None();

        let a = Option::isSome(some);
        let b = Option::isNone(some);
        let c = Option::isSome(none);
        let d = Option::unwrapOr(some, 0);
        let e = Option::unwrapOr(none, 10);

        // encode results into an array so we can check all at once
        [a, b, c, d, e];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Boolean(true));  // isSome(Some)
            assert_eq!(vals[1], Object::Boolean(false)); // isNone(Some)
            assert_eq!(vals[2], Object::Boolean(false)); // isSome(None)
            assert_eq!(vals[3], Object::Integer(5));      // unwrapOr(Some(5), 0)
            assert_eq!(vals[4], Object::Integer(10));     // unwrapOr(None, 10)
        }
        other => panic!("expected array from option helper test, got {:?}", other),
    }
}

#[test]
fn test_result_helpers() {
    let input = r#"
        let ok = Result::Ok(7);
        let err = Result::Err("boom");

        let a = Result::isOk(ok);
        let b = Result::isErr(ok);
        let c = Result::isOk(err);
        let d = Result::unwrapOr(ok, 0);
        let e = Result::unwrapOr(err, 10);

        [a, b, c, d, e];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Boolean(true));  // isOk(Ok)
            assert_eq!(vals[1], Object::Boolean(false)); // isErr(Ok)
            assert_eq!(vals[2], Object::Boolean(false)); // isOk(Err)
            assert_eq!(vals[3], Object::Integer(7));      // unwrapOr(Ok(7), 0)
            assert_eq!(vals[4], Object::Integer(10));     // unwrapOr(Err(_), 10)
        }
        other => panic!("expected array from result helper test, got {:?}", other),
    }
}

#[test]
fn test_option_map_and_then() {
    let input = r#"
        let inc = fn(x) { x + 1; };
        let to_opt = fn(x) {
            if (x > 0) {
                Option::Some(x);
            } else {
                Option::None();
            }
        };

        let a = Option::map(Option::Some(1), inc);      // Some(2)
        let b = Option::map(Option::None(), inc);       // None

        let c = Option::andThen(Option::Some(1), to_opt); // Some(1)
        let d = Option::andThen(Option::Some(0), to_opt); // None

        [a, b, c, d];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);

            match &vals[0] {
                Object::OptionSome(inner) => assert_eq!(**inner, Object::Integer(2)),
                other => panic!("expected Option::Some(2) for a, got {:?}", other),
            }

            assert_eq!(vals[1], Object::OptionNone);

            match &vals[2] {
                Object::OptionSome(inner) => assert_eq!(**inner, Object::Integer(1)),
                other => panic!("expected Option::Some(1) for c, got {:?}", other),
            }

            assert_eq!(vals[3], Object::OptionNone);
        }
        other => panic!("expected array from option map/and_then test, got {:?}", other),
    }
}

#[test]
fn test_result_map_and_then() {
    let input = r#"
        let inc = fn(x) { x + 1; };
        let to_res = fn(x) {
            if (x > 0) {
                Result::Ok(x);
            } else {
                Result::Err("non-positive");
            }
        };

        let a = Result::map(Result::Ok(1), inc);      // Ok(2)
        let b = Result::map(Result::Err("e"), inc);   // Err("e")

        let c = Result::andThen(Result::Ok(1), to_res); // Ok(1)
        let d = Result::andThen(Result::Ok(0), to_res); // Err("non-positive")

        [a, b, c, d];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);

            match &vals[0] {
                Object::ResultOk(inner) => assert_eq!(**inner, Object::Integer(2)),
                other => panic!("expected Result::Ok(2) for a, got {:?}", other),
            }

            match &vals[1] {
                Object::ResultErr(inner) => match &**inner {
                    Object::String(s) => assert_eq!(s, "e"),
                    v => panic!("expected inner string \"e\" for b, got {:?}", v),
                },
                other => panic!("expected Result::Err(\"e\") for b, got {:?}", other),
            }

            match &vals[2] {
                Object::ResultOk(inner) => assert_eq!(**inner, Object::Integer(1)),
                other => panic!("expected Result::Ok(1) for c, got {:?}", other),
            }

            match &vals[3] {
                Object::ResultErr(inner) => match &**inner {
                    Object::String(s) => assert_eq!(s, "non-positive"),
                    v => panic!("expected inner string \"non-positive\" for d, got {:?}", v),
                },
                other => panic!("expected Result::Err(\"non-positive\") for d, got {:?}", other),
            }
        }
        other => panic!("expected array from result map/and_then test, got {:?}", other),
    }
}

#[test]
fn test_file_namespace_result_helpers() {
    let input = r#"
        // Create a fresh file and write to it using the File:: API
        let opened = File::open("tmp_file_namespace_ok.txt", "w+");
        let f = Result::unwrapOr(opened, 0);

        let _ = File::write(f, "Hello, world!");
        let _ = File::seek(f, 0, "start");

        let contentsResult = File::read(f);

        let contents = Result::unwrapOr(contentsResult, "ERR");
        contents;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::String(s) => assert_eq!(s, "Hello, world!"),
        other => panic!("expected file contents string, got {:?}", other),
    }
}

#[test]
fn test_file_namespace_open_errors() {
    let input = r#"
        let res1 = File::open("this_file_does_not_exist_xyz.txt", "r");
        let res2 = File::open("tmp_file_namespace_open_mode.txt", "badmode");

        let a = Result::isOk(res1);
        let b = Result::isErr(res1);
        let c = Result::isOk(res2);
        let d = Result::isErr(res2);

        [a, b, c, d];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Boolean(false)); // res1 is not Ok
            assert_eq!(vals[1], Object::Boolean(true));  // res1 is Err
            assert_eq!(vals[2], Object::Boolean(false)); // res2 is not Ok
            assert_eq!(vals[3], Object::Boolean(true));  // res2 is Err
        }
        other => panic!("expected array from file open error test, got {:?}", other),
    }
}

#[test]
fn test_file_namespace_read_write_errors() {
    let input = r#"
        // Open a real file; if this fails we still expect subsequent calls
        // to produce errors, but in normal test runs it should succeed.
        let opened = File::open("tmp_file_namespace_errors.txt", "w+");
        let f = Result::unwrapOr(opened, 0);

        // Using non-file as first argument
        let res1 = File::read(123);
        let res2 = File::write(123, "data");

        // Wrong type for data argument
        let res3 = File::write(f, 42);

        // Closed file errors
        let _ = File::close(f);
        let res4 = File::read(f);

        let a = Result::isErr(res1);
        let b = Result::isErr(res2);
        let c = Result::isErr(res3);
        let d = Result::isErr(res4);

        [a, b, c, d];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            // All four scenarios should produce Result::Err(...)
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(true));
        }
        other => panic!("expected array from file read/write error test, got {:?}", other),
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


