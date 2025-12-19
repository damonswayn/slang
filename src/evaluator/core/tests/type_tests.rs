use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_type_of() {
    let input = r#"
        let t1 = Type::of(42);
        let t2 = Type::of(3.14);
        let t3 = Type::of(true);
        let t4 = Type::of("hello");
        let t5 = Type::of([1, 2, 3]);
        let t6 = Type::of({ a: 1 });
        let t7 = Type::of(fn(x) { x; });
        let t8 = Type::of(Option::Some(1));
        let t9 = Type::of(Option::None());
        let t10 = Type::of(Result::Ok(1));

        [t1, t2, t3, t4, t5, t6, t7, t8, t9, t10];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 10);
            assert_eq!(vals[0], Object::String("integer".to_string()));
            assert_eq!(vals[1], Object::String("float".to_string()));
            assert_eq!(vals[2], Object::String("boolean".to_string()));
            assert_eq!(vals[3], Object::String("string".to_string()));
            assert_eq!(vals[4], Object::String("array".to_string()));
            assert_eq!(vals[5], Object::String("object".to_string()));
            assert_eq!(vals[6], Object::String("function".to_string()));
            assert_eq!(vals[7], Object::String("option".to_string()));
            assert_eq!(vals[8], Object::String("option".to_string()));
            assert_eq!(vals[9], Object::String("result".to_string()));
        }
        other => panic!("expected array from Type::of test, got {:?}", other),
    }
}

#[test]
fn test_type_is_checks() {
    let input = r#"
        let i1 = Type::isInt(42);
        let i2 = Type::isInt(3.14);
        let i3 = Type::isInt("42");

        let f1 = Type::isFloat(3.14);
        let f2 = Type::isFloat(42);

        let n1 = Type::isNumber(42);
        let n2 = Type::isNumber(3.14);
        let n3 = Type::isNumber("42");

        let b1 = Type::isBool(true);
        let b2 = Type::isBool(1);

        let s1 = Type::isString("hello");
        let s2 = Type::isString(123);

        [i1, i2, i3, f1, f2, n1, n2, n3, b1, b2, s1, s2];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 12);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Boolean(false));
            assert_eq!(vals[3], Object::Boolean(true));
            assert_eq!(vals[4], Object::Boolean(false));
            assert_eq!(vals[5], Object::Boolean(true));
            assert_eq!(vals[6], Object::Boolean(true));
            assert_eq!(vals[7], Object::Boolean(false));
            assert_eq!(vals[8], Object::Boolean(true));
            assert_eq!(vals[9], Object::Boolean(false));
            assert_eq!(vals[10], Object::Boolean(true));
            assert_eq!(vals[11], Object::Boolean(false));
        }
        other => panic!("expected array from Type::is* test, got {:?}", other),
    }
}

#[test]
fn test_type_is_compound_checks() {
    let input = r#"
        let a1 = Type::isArray([1, 2, 3]);
        let a2 = Type::isArray({ a: 1 });

        let o1 = Type::isObject({ a: 1 });
        let o2 = Type::isObject([1, 2, 3]);

        let c1 = Type::isCallable(fn(x) { x; });
        let c2 = Type::isCallable(len);
        let c3 = Type::isCallable(42);

        let it1 = Type::isIterable([1, 2, 3]);
        let it2 = Type::isIterable("hello");
        let it3 = Type::isIterable(42);

        [a1, a2, o1, o2, c1, c2, c3, it1, it2, it3];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 10);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(false));
            assert_eq!(vals[4], Object::Boolean(true));
            assert_eq!(vals[5], Object::Boolean(true));
            assert_eq!(vals[6], Object::Boolean(false));
            assert_eq!(vals[7], Object::Boolean(true));
            assert_eq!(vals[8], Object::Boolean(true));
            assert_eq!(vals[9], Object::Boolean(false));
        }
        other => panic!("expected array from Type::is* compound test, got {:?}", other),
    }
}

#[test]
fn test_type_is_special_checks() {
    let input = r#"
        let arr = [1];
        let nullVal = arr[10];

        let n1 = Type::isNull(nullVal);
        let n2 = Type::isNull(0);

        let op1 = Type::isOption(Option::Some(1));
        let op2 = Type::isOption(Option::None());
        let op3 = Type::isOption(42);

        let r1 = Type::isResult(Result::Ok(1));
        let r2 = Type::isResult(Result::Err("oops"));
        let r3 = Type::isResult(42);

        [n1, n2, op1, op2, op3, r1, r2, r3];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 8);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(true));
            assert_eq!(vals[4], Object::Boolean(false));
            assert_eq!(vals[5], Object::Boolean(true));
            assert_eq!(vals[6], Object::Boolean(true));
            assert_eq!(vals[7], Object::Boolean(false));
        }
        other => panic!("expected array from Type::is* special test, got {:?}", other),
    }
}

