use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_fn_identity() {
    let input = r#"
        let a = Fn::identity(42);
        let b = Fn::identity("hello");
        let c = Fn::identity([1, 2, 3]);
        [a, b, c];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(42));
            assert_eq!(vals[1], Object::String("hello".to_string()));
            match &vals[2] {
                Object::Array(arr) => assert_eq!(arr.len(), 3),
                other => panic!("expected array, got {:?}", other),
            }
        }
        other => panic!("expected array from Fn::identity test, got {:?}", other),
    }
}

#[test]
fn test_fn_constant() {
    let input = r#"
        let always42 = Fn::constant(42);
        let v1 = Fn::call(always42);
        let v2 = Fn::call(always42, "ignored", 1, 2, 3);
        [v1, v2];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(42));
            assert_eq!(vals[1], Object::Integer(42));
        }
        other => panic!("expected array from Fn::constant test, got {:?}", other),
    }
}

#[test]
fn test_fn_compose() {
    let input = r#"
        let double = fn(x) { x * 2 };
        let addOne = fn(x) { x + 1 };
        let doubleThenAddOne = Fn::compose(addOne, double);
        Fn::call(doubleThenAddOne, 5);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(11));
}

#[test]
fn test_fn_pipe() {
    let input = r#"
        let double = fn(x) { x * 2 };
        let addOne = fn(x) { x + 1 };
        let addOneThenDouble = Fn::pipe(addOne, double);
        Fn::call(addOneThenDouble, 5);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(12));
}

#[test]
fn test_fn_apply() {
    let input = r#"
        let add = fn(a, b) { a + b };
        Fn::apply(add, [3, 4]);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(7));
}

#[test]
fn test_fn_call() {
    let input = r#"
        let multiply = fn(a, b, c) { a * b * c };
        Fn::call(multiply, 2, 3, 4);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(24));
}

#[test]
fn test_fn_negate() {
    let input = r#"
        let isEven = fn(x) { x % 2 == 0 };
        let isOdd = Fn::negate(isEven);
        let a = Fn::call(isOdd, 3);
        let b = Fn::call(isOdd, 4);
        [a, b];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
        }
        other => panic!("expected array from Fn::negate test, got {:?}", other),
    }
}

#[test]
fn test_fn_flip() {
    let input = r#"
        let subtract = fn(a, b) { a - b };
        let flipped = Fn::flip(subtract);
        let normal = subtract(10, 3);
        let reversed = Fn::call(flipped, 10, 3);
        [normal, reversed];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(7));
            assert_eq!(vals[1], Object::Integer(-7));
        }
        other => panic!("expected array from Fn::flip test, got {:?}", other),
    }
}

#[test]
fn test_fn_partial() {
    let input = r#"
        let add3 = fn(a, b, c) { a + b + c };
        let addTo10 = Fn::partial(add3, 10);
        Fn::call(addTo10, 5, 3);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(18));
}

#[test]
fn test_fn_is_callable() {
    let input = r#"
        let f = fn(x) { x };
        let a = Fn::isCallable(f);
        let b = Fn::isCallable(len);
        let c = Fn::isCallable(42);
        let d = Fn::isCallable("hello");
        [a, b, c, d];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(false));
            assert_eq!(vals[3], Object::Boolean(false));
        }
        other => panic!("expected array from Fn::isCallable test, got {:?}", other),
    }
}

#[test]
fn test_fn_error_handling() {
    let input = r#"Fn::identity();"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from Fn::identity with no args, got {:?}", other),
    }

    let input2 = r#"Fn::compose(42, fn(x) { x });"#;
    let obj2 = eval_input(input2);
    match obj2 {
        Object::Error(_) => {}
        other => panic!("expected error from Fn::compose with non-callable, got {:?}", other),
    }

    let input3 = r#"Fn::apply(fn(x) { x }, "not an array");"#;
    let obj3 = eval_input(input3);
    match obj3 {
        Object::Error(_) => {}
        other => panic!("expected error from Fn::apply with non-array, got {:?}", other),
    }
}



