use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_json_namespace_parse_and_stringify() {
    let input = r#"
        let s = "[1, true, null, 3.5]";
        let parsedArr = Json::parse(s);
        let arr = Result::unwrapOr(parsedArr, 0);

        let a0 = arr[0];
        let a1 = arr[1];
        let a2 = arr[2];
        let a3 = arr[3];

        let obj = { a: 1, b: [true, 3.5] };
        let roundTrip = Json::stringify(obj);
        let rtStr = Result::unwrapOr(roundTrip, "ERR");

        [a0, a1, a2, a3, rtStr];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);

            assert_eq!(vals[0], Object::Integer(1));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Null);
            match &vals[3] {
                Object::Float(f) => assert!((*f - 3.5).abs() < 1e-9),
                other => panic!("expected float 3.5, got {:?}", other),
            }

            match &vals[4] {
                Object::String(s) => {
                    assert_eq!(s, "{\"a\":1,\"b\":[true,3.5]}");
                }
                other => panic!("expected JSON string from Json::stringify, got {:?}", other),
            }
        }
        other => panic!("expected array from Json namespace test, got {:?}", other),
    }
}
