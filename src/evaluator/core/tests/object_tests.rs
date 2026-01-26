use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_object_namespace_keys_values_entries() {
    let input = r#"
        let obj = { name: "Alice", age: 30, active: true };

        let keys = Obj::keys(obj);
        let values = Obj::values(obj);
        let entries = Obj::entries(obj);

        let keyCount = len(keys);
        let valueCount = len(values);
        let entryCount = len(entries);

        [keyCount, valueCount, entryCount];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(3));
            assert_eq!(vals[1], Object::Integer(3));
            assert_eq!(vals[2], Object::Integer(3));
        }
        other => panic!(
            "expected array from Obj::keys/values/entries test, got {:?}",
            other
        ),
    }
}

#[test]
fn test_object_namespace_from_entries() {
    let input = r#"
        let entries = [["a", 1], ["b", 2], ["c", 3]];
        let obj = Obj::fromEntries(entries);

        let a = obj.a;
        let b = obj.b;
        let c = obj.c;

        [a, b, c];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(1));
            assert_eq!(vals[1], Object::Integer(2));
            assert_eq!(vals[2], Object::Integer(3));
        }
        other => panic!("expected array from Obj::fromEntries test, got {:?}", other),
    }
}

#[test]
fn test_object_namespace_has_and_get() {
    let input = r#"
        let obj = { name: "Bob", age: 25 };

        let hasName = Obj::has(obj, "name");
        let hasCity = Obj::has(obj, "city");

        let getName = Obj::get(obj, "name");
        let getCity = Obj::get(obj, "city");

        let nameValue = Option::unwrapOr(getName, "default");
        let cityValue = Option::unwrapOr(getCity, "unknown");

        [hasName, hasCity, nameValue, cityValue];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::String("Bob".to_string()));
            assert_eq!(vals[3], Object::String("unknown".to_string()));
        }
        other => panic!("expected array from Obj::has/get test, got {:?}", other),
    }
}

#[test]
fn test_object_namespace_set_and_delete() {
    let input = r#"
        let obj = { a: 1, b: 2 };

        let withC = Obj::set(obj, "c", 3);
        let withoutA = Obj::delete(obj, "a");

        let originalHasC = Obj::has(obj, "c");
        let newHasC = Obj::has(withC, "c");
        let cValue = withC.c;

        let originalHasA = Obj::has(obj, "a");
        let deletedHasA = Obj::has(withoutA, "a");

        [originalHasC, newHasC, cValue, originalHasA, deletedHasA];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Boolean(false));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Integer(3));
            assert_eq!(vals[3], Object::Boolean(true));
            assert_eq!(vals[4], Object::Boolean(false));
        }
        other => panic!("expected array from Obj::set/delete test, got {:?}", other),
    }
}

#[test]
fn test_object_namespace_merge() {
    let input = r#"
        let obj1 = { a: 1, b: 2 };
        let obj2 = { b: 20, c: 30 };

        let merged = Obj::merge(obj1, obj2);

        let a = merged.a;
        let b = merged.b;
        let c = merged.c;

        [a, b, c];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(1));
            assert_eq!(vals[1], Object::Integer(20));
            assert_eq!(vals[2], Object::Integer(30));
        }
        other => panic!("expected array from Obj::merge test, got {:?}", other),
    }
}

#[test]
fn test_object_namespace_is_empty_and_len() {
    let input = r#"
        let empty = {};
        let nonEmpty = { x: 1, y: 2 };

        let isEmptyEmpty = Obj::isEmpty(empty);
        let isEmptyNonEmpty = Obj::isEmpty(nonEmpty);

        let lenEmpty = Obj::len(empty);
        let lenNonEmpty = Obj::len(nonEmpty);

        [isEmptyEmpty, isEmptyNonEmpty, lenEmpty, lenNonEmpty];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Integer(0));
            assert_eq!(vals[3], Object::Integer(2));
        }
        other => panic!("expected array from Obj::isEmpty/len test, got {:?}", other),
    }
}

#[test]
fn test_object_namespace_roundtrip() {
    let input = r#"
        let original = { name: "Test", value: 42, flag: true };
        let entries = Obj::entries(original);
        let reconstructed = Obj::fromEntries(entries);

        let nameMatch = reconstructed.name == original.name;
        let valueMatch = reconstructed.value == original.value;
        let flagMatch = reconstructed.flag == original.flag;

        [nameMatch, valueMatch, flagMatch];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(true));
        }
        other => panic!("expected array from Obj roundtrip test, got {:?}", other),
    }
}

#[test]
fn test_object_namespace_error_handling() {
    let input1 = r#"Obj::keys([1, 2, 3]);"#;
    let obj1 = eval_input(input1);
    assert!(obj1.is_error(), "Obj::keys on array should error");

    let input2 = r#"Obj::has({ a: 1 }, 123);"#;
    let obj2 = eval_input(input2);
    assert!(obj2.is_error(), "Obj::has with non-string key should error");

    let input3 = r#"Obj::fromEntries([["a", 1], "not-a-pair"]);"#;
    let obj3 = eval_input(input3);
    assert!(
        obj3.is_error(),
        "Obj::fromEntries with invalid entry should error"
    );
}
