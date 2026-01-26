use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_regex_builtins() {
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

            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));

            match &vals[2] {
                Object::OptionSome(inner) => match &**inner {
                    Object::String(s) => assert_eq!(s, "123"),
                    other => panic!("expected inner string \"123\" for m1, got {:?}", other),
                },
                other => panic!("expected Option::Some(\"123\") for m1, got {:?}", other),
            }

            assert_eq!(vals[3], Object::OptionNone);

            match &vals[4] {
                Object::String(s) => assert_eq!(s, "foo X bar X"),
                other => panic!("expected replaced string, got {:?}", other),
            }

            match &vals[5] {
                Object::OptionSome(inner) => match &**inner {
                    Object::Array(groups) => {
                        assert_eq!(groups.len(), 3);
                        assert_eq!(groups[0], Object::String("abc123".to_string()));
                        assert_eq!(groups[1], Object::String("abc".to_string()));
                        assert_eq!(groups[2], Object::String("123".to_string()));
                    }
                    other => panic!("expected array of groups for c1, got {:?}", other),
                },
                other => panic!("expected Option::Some([...]) for c1, got {:?}", other),
            }

            assert_eq!(vals[6], Object::OptionNone);
        }
        other => panic!("expected array from regex builtins test, got {:?}", other),
    }
}
