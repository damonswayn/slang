use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_option_constructors() {
    let some = eval_input("Option::Some(5);");
    match some {
        Object::OptionSome(inner) => assert_eq!(*inner, Object::Integer(5)),
        other => panic!("expected Option::Some(5), got {:?}", other),
    }

    let none = eval_input("Option::None();");
    assert_eq!(none, Object::OptionNone);
}

#[test]
fn test_result_constructors() {
    let ok = eval_input("Result::Ok(42);");
    match ok {
        Object::ResultOk(inner) => assert_eq!(*inner, Object::Integer(42)),
        other => panic!("expected Result::Ok(42), got {:?}", other),
    }

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

        [a, b, c, d, e];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Boolean(false));
            assert_eq!(vals[3], Object::Integer(5));
            assert_eq!(vals[4], Object::Integer(10));
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
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Boolean(false));
            assert_eq!(vals[3], Object::Integer(7));
            assert_eq!(vals[4], Object::Integer(10));
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

        let a = Option::map(Option::Some(1), inc);
        let b = Option::map(Option::None(), inc);

        let c = Option::andThen(Option::Some(1), to_opt);
        let d = Option::andThen(Option::Some(0), to_opt);

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

        let a = Result::map(Result::Ok(1), inc);
        let b = Result::map(Result::Err("e"), inc);

        let c = Result::andThen(Result::Ok(1), to_res);
        let d = Result::andThen(Result::Ok(0), to_res);

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



