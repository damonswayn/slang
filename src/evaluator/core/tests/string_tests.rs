use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_string_namespace_basic() {
    let input = r#"
        let s = "  Hello World  ";
        let t = String::trim(s);
        let upper = String::toUpper(t);
        let lower = String::toLower(t);

        let parts = String::split("a,b,c", ",");
        let chars = String::split("hi", "");
        let joined = String::join(["x", "y", "z"], "-");

        [t, upper, lower, parts, chars, joined];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);

            assert_eq!(vals[0], Object::String("Hello World".to_string()));
            assert_eq!(vals[1], Object::String("HELLO WORLD".to_string()));
            assert_eq!(vals[2], Object::String("hello world".to_string()));

            match &vals[3] {
                Object::Array(parts) => {
                    assert_eq!(parts.len(), 3);
                    assert_eq!(parts[0], Object::String("a".to_string()));
                    assert_eq!(parts[1], Object::String("b".to_string()));
                    assert_eq!(parts[2], Object::String("c".to_string()));
                }
                other => panic!("expected array from String::split, got {:?}", other),
            }

            match &vals[4] {
                Object::Array(chars) => {
                    assert_eq!(chars.len(), 2);
                    assert_eq!(chars[0], Object::String("h".to_string()));
                    assert_eq!(chars[1], Object::String("i".to_string()));
                }
                other => panic!("expected array of chars from String::split with empty sep, got {:?}", other),
            }

            assert_eq!(vals[5], Object::String("x-y-z".to_string()));
        }
        other => panic!("expected array from String namespace test, got {:?}", other),
    }
}

#[test]
fn test_string_contains_starts_ends() {
    let input = r#"
        let s = "hello world";

        let c1 = String::contains(s, "world");
        let c2 = String::contains(s, "foo");

        let sw1 = String::startsWith(s, "hello");
        let sw2 = String::startsWith(s, "world");

        let ew1 = String::endsWith(s, "world");
        let ew2 = String::endsWith(s, "hello");

        [c1, c2, sw1, sw2, ew1, ew2];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(false));
            assert_eq!(vals[4], Object::Boolean(true));
            assert_eq!(vals[5], Object::Boolean(false));
        }
        other => panic!("expected array from String::contains/startsWith/endsWith test, got {:?}", other),
    }
}

#[test]
fn test_string_index_of() {
    let input = r#"
        let s = "hello world";

        let idx1 = String::indexOf(s, "world");
        let idx2 = String::indexOf(s, "o");
        let idx3 = String::indexOf(s, "xyz");

        let val1 = Option::unwrapOr(idx1, -1);
        let val2 = Option::unwrapOr(idx2, -1);
        let val3 = Option::unwrapOr(idx3, -1);

        [val1, val2, val3, Option::isNone(idx3)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Integer(6));
            assert_eq!(vals[1], Object::Integer(4));
            assert_eq!(vals[2], Object::Integer(-1));
            assert_eq!(vals[3], Object::Boolean(true));
        }
        other => panic!("expected array from String::indexOf test, got {:?}", other),
    }
}

#[test]
fn test_string_slice() {
    let input = r#"
        let s = "hello world";

        let s1 = String::slice(s, 0, 5);
        let s2 = String::slice(s, 6, 11);
        let s3 = String::slice(s, -5, 11);
        let s4 = String::slice(s, 0, -6);
        let s5 = String::slice(s, 3, 3);
        let s6 = String::slice(s, 5, 2);

        [s1, s2, s3, s4, s5, s6];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::String("hello".to_string()));
            assert_eq!(vals[1], Object::String("world".to_string()));
            assert_eq!(vals[2], Object::String("world".to_string()));
            assert_eq!(vals[3], Object::String("hello".to_string()));
            assert_eq!(vals[4], Object::String("".to_string()));
            assert_eq!(vals[5], Object::String("".to_string()));
        }
        other => panic!("expected array from String::slice test, got {:?}", other),
    }
}

#[test]
fn test_string_replace() {
    let input = r#"
        let s = "hello hello world";

        let r1 = String::replace(s, "hello", "hi");
        let r2 = String::replace(s, "world", "universe");
        let r3 = String::replace(s, "xyz", "abc");

        [r1, r2, r3];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::String("hi hello world".to_string()));
            assert_eq!(vals[1], Object::String("hello hello universe".to_string()));
            assert_eq!(vals[2], Object::String("hello hello world".to_string()));
        }
        other => panic!("expected array from String::replace test, got {:?}", other),
    }
}

#[test]
fn test_string_repeat() {
    let input = r#"
        let r1 = String::repeat("ab", 3);
        let r2 = String::repeat("x", 5);
        let r3 = String::repeat("hello", 0);
        let r4 = String::repeat("", 10);

        [r1, r2, r3, r4];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::String("ababab".to_string()));
            assert_eq!(vals[1], Object::String("xxxxx".to_string()));
            assert_eq!(vals[2], Object::String("".to_string()));
            assert_eq!(vals[3], Object::String("".to_string()));
        }
        other => panic!("expected array from String::repeat test, got {:?}", other),
    }
}

#[test]
fn test_string_reverse() {
    let input = r#"
        let r1 = String::reverse("hello");
        let r2 = String::reverse("12345");
        let r3 = String::reverse("");
        let r4 = String::reverse("a");

        [r1, r2, r3, r4];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::String("olleh".to_string()));
            assert_eq!(vals[1], Object::String("54321".to_string()));
            assert_eq!(vals[2], Object::String("".to_string()));
            assert_eq!(vals[3], Object::String("a".to_string()));
        }
        other => panic!("expected array from String::reverse test, got {:?}", other),
    }
}

#[test]
fn test_string_pad_left_right() {
    let input = r#"
        let pl1 = String::padLeft("42", 5, "0");
        let pl2 = String::padLeft("hello", 3, "x");
        let pl3 = String::padLeft("a", 4, " ");

        let pr1 = String::padRight("42", 5, "0");
        let pr2 = String::padRight("hello", 3, "x");
        let pr3 = String::padRight("a", 4, "-");

        [pl1, pl2, pl3, pr1, pr2, pr3];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::String("00042".to_string()));
            assert_eq!(vals[1], Object::String("hello".to_string()));
            assert_eq!(vals[2], Object::String("   a".to_string()));
            assert_eq!(vals[3], Object::String("42000".to_string()));
            assert_eq!(vals[4], Object::String("hello".to_string()));
            assert_eq!(vals[5], Object::String("a---".to_string()));
        }
        other => panic!("expected array from String::padLeft/padRight test, got {:?}", other),
    }
}

#[test]
fn test_string_namespace_error_handling() {
    let input1 = r#"String::contains(123, "x");"#;
    let obj1 = eval_input(input1);
    assert!(obj1.is_error(), "String::contains with non-string should error");

    let input2 = r#"String::slice("hello", "a", 3);"#;
    let obj2 = eval_input(input2);
    assert!(obj2.is_error(), "String::slice with non-integer start should error");

    let input3 = r#"String::repeat("x", -1);"#;
    let obj3 = eval_input(input3);
    assert!(obj3.is_error(), "String::repeat with negative count should error");

    let input4 = r#"String::padLeft("x", 5, "ab");"#;
    let obj4 = eval_input(input4);
    assert!(obj4.is_error(), "String::padLeft with multi-char pad should error");
}

#[test]
fn test_string_chars() {
    let input = r#"
        let chars = String::chars("hello");
        chars;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::String("h".to_string()));
            assert_eq!(vals[1], Object::String("e".to_string()));
            assert_eq!(vals[2], Object::String("l".to_string()));
            assert_eq!(vals[3], Object::String("l".to_string()));
            assert_eq!(vals[4], Object::String("o".to_string()));
        }
        other => panic!("expected array from String::chars, got {:?}", other),
    }
}

#[test]
fn test_string_chars_unicode() {
    let input = r#"
        let chars = String::chars("日本語");
        chars;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::String("日".to_string()));
            assert_eq!(vals[1], Object::String("本".to_string()));
            assert_eq!(vals[2], Object::String("語".to_string()));
        }
        other => panic!("expected array from String::chars with unicode, got {:?}", other),
    }
}

#[test]
fn test_string_char_code_at() {
    let input = r#"
        let a = String::charCodeAt("ABC", 0);
        let b = String::charCodeAt("ABC", 1);
        let c = String::charCodeAt("ABC", 2);
        [a, b, c];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(65));
            assert_eq!(vals[1], Object::Integer(66));
            assert_eq!(vals[2], Object::Integer(67));
        }
        other => panic!("expected array from String::charCodeAt, got {:?}", other),
    }
}

#[test]
fn test_string_from_char_code() {
    let input = r#"
        let a = String::fromCharCode(65);
        let b = String::fromCharCode(66);
        let c = String::fromCharCode(67);
        [a, b, c];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::String("A".to_string()));
            assert_eq!(vals[1], Object::String("B".to_string()));
            assert_eq!(vals[2], Object::String("C".to_string()));
        }
        other => panic!("expected array from String::fromCharCode, got {:?}", other),
    }
}

#[test]
fn test_string_from_char_codes() {
    let input = r#"
        String::fromCharCodes([72, 101, 108, 108, 111]);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("Hello".to_string()));
}

#[test]
fn test_string_char_codes() {
    let input = r#"
        String::charCodes("ABC");
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(65));
            assert_eq!(vals[1], Object::Integer(66));
            assert_eq!(vals[2], Object::Integer(67));
        }
        other => panic!("expected array from String::charCodes, got {:?}", other),
    }
}

#[test]
fn test_string_last_index_of() {
    let input = r#"
        let s = "hello world hello";
        let idx1 = String::lastIndexOf(s, "hello");
        let idx2 = String::lastIndexOf(s, "world");
        let idx3 = String::lastIndexOf(s, "xyz");
        
        let val1 = Option::unwrapOr(idx1, -1);
        let val2 = Option::unwrapOr(idx2, -1);
        let val3 = Option::unwrapOr(idx3, -1);
        
        [val1, val2, Option::isNone(idx3)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(12));
            assert_eq!(vals[1], Object::Integer(6));
            assert_eq!(vals[2], Object::Boolean(true));
        }
        other => panic!("expected array from String::lastIndexOf, got {:?}", other),
    }
}

#[test]
fn test_string_replace_all() {
    let input = r#"
        let s = "hello world hello world";
        String::replaceAll(s, "world", "universe");
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("hello universe hello universe".to_string()));
}

#[test]
fn test_string_is_empty() {
    let input = r#"
        let a = String::isEmpty("");
        let b = String::isEmpty("hello");
        [a, b];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
        }
        other => panic!("expected array from String::isEmpty, got {:?}", other),
    }
}

#[test]
fn test_string_len() {
    let input = r#"
        let a = String::len("");
        let b = String::len("hello");
        let c = String::len("日本語");
        [a, b, c];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(0));
            assert_eq!(vals[1], Object::Integer(5));
            assert_eq!(vals[2], Object::Integer(3));
        }
        other => panic!("expected array from String::len, got {:?}", other),
    }
}

#[test]
fn test_string_extras_error_handling() {
    let input = r#"String::charCodeAt("hi", 10);"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from String::charCodeAt out of bounds, got {:?}", other),
    }

    let input2 = r#"String::fromCharCode(-1);"#;
    let obj2 = eval_input(input2);
    match obj2 {
        Object::Error(_) => {}
        other => panic!("expected error from String::fromCharCode negative, got {:?}", other),
    }

    let input3 = r#"String::chars(123);"#;
    let obj3 = eval_input(input3);
    match obj3 {
        Object::Error(_) => {}
        other => panic!("expected error from String::chars with non-string, got {:?}", other),
    }
}




