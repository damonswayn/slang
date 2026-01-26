use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_class_definition() {
    let input = r#"
        class Foo {
            function bar() { 42; }
        }
        Type::of(Foo);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("class".to_string()));
}

#[test]
fn test_new_creates_instance() {
    let input = r#"
        class Empty {}
        let e = new Empty();
        Type::of(e);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("object".to_string()));
}

#[test]
fn test_instance_has_methods() {
    let input = r#"
        class Calculator {
            function add(a, b) { a + b; }
            function multiply(a, b) { a * b; }
        }
        let calc = new Calculator();
        [calc.add(2, 3), calc.multiply(4, 5)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(20));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

#[test]
fn test_constructor_called() {
    // Verify constructor is called by checking that it sets a field on this
    let input = r#"
        class Tracker {
            function construct() {
                this.initialized = true;
            }
        }
        let t = new Tracker();
        t.initialized;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_constructor_with_arguments() {
    // Verify constructor receives arguments by checking field values
    let input = r#"
        class Point {
            function construct(x, y) {
                this.x = x;
                this.y = y;
            }
        }
        let p = new Point(10, 20);
        [p.x, p.y];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(10));
            assert_eq!(vals[1], Object::Integer(20));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

#[test]
fn test_constructor_sets_fields() {
    let input = r#"
        class Person {
            function construct(name, age) {
                this.name = name;
                this.age = age;
            }
        }
        let p = new Person("Alice", 30);
        [p.name, p.age];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::String("Alice".to_string()));
            assert_eq!(vals[1], Object::Integer(30));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

#[test]
fn test_method_accesses_this() {
    let input = r#"
        class Counter {
            function construct(start) {
                this.value = start;
            }
            function getValue() {
                this.value;
            }
        }
        let c = new Counter(42);
        c.getValue();
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(42));
}

#[test]
fn test_method_modifies_this() {
    // Currently, methods need to return `this` to propagate mutations.
    // This test verifies that pattern works correctly.
    let input = r#"
        class Counter {
            function construct(start) {
                this.value = start;
            }
            function increment() {
                this.value = this.value + 1;
                this;
            }
            function getValue() {
                this.value;
            }
        }
        let c = new Counter(0);
        c = c.increment();
        c = c.increment();
        c = c.increment();
        c.getValue();
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(3));
}

#[test]
fn test_class_without_constructor() {
    let input = r#"
        class Utils {
            function double(x) { x * 2; }
            function triple(x) { x * 3; }
        }
        let u = new Utils();
        [u.double(5), u.triple(5)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(10));
            assert_eq!(vals[1], Object::Integer(15));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

#[test]
fn test_multiple_instances() {
    let input = r#"
        class Box {
            function construct(val) {
                this.value = val;
            }
            function get() {
                this.value;
            }
        }
        let a = new Box(100);
        let b = new Box(200);
        [a.get(), b.get()];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(100));
            assert_eq!(vals[1], Object::Integer(200));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

#[test]
fn test_class_not_found_error() {
    let input = r#"new NonExistentClass();"#;
    let obj = eval_input(input);
    assert!(obj.is_error(), "expected error for unknown class");
}

#[test]
fn test_new_on_non_class_error() {
    let input = r#"
        let notAClass = 42;
        new notAClass();
    "#;
    let obj = eval_input(input);
    assert!(obj.is_error(), "expected error when using new on non-class");
}

#[test]
fn test_method_chaining() {
    let input = r#"
        class Builder {
            function construct() {
                this.value = 0;
            }
            function add(n) {
                this.value = this.value + n;
                this;
            }
            function multiply(n) {
                this.value = this.value * n;
                this;
            }
            function result() {
                this.value;
            }
        }
        let b = new Builder();
        b.add(5).multiply(2).add(3).result();
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(13));
}

#[test]
fn test_class_with_complex_method() {
    let input = r#"
        class Math {
            function factorial(n) {
                if (n <= 1) {
                    1;
                } else {
                    n * this.factorial(n - 1);
                }
            }
        }
        let m = new Math();
        m.factorial(5);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(120));
}
