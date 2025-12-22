use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_time_now() {
    let input = r#"
        let ts = Time::now();
        ts > 0;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_time_now_secs() {
    let input = r#"
        let ts = Time::nowSecs();
        ts > 0;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_time_components() {
    let input = r#"
        let ts = 1718458245000;
        let year = Time::year(ts);
        let month = Time::month(ts);
        let day = Time::day(ts);
        let hour = Time::hour(ts);
        let minute = Time::minute(ts);
        let second = Time::second(ts);
        [year, month, day, hour, minute, second];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Integer(2024));
            assert_eq!(vals[1], Object::Integer(6));
            assert_eq!(vals[2], Object::Integer(15));
            assert_eq!(vals[3], Object::Integer(13));
            assert_eq!(vals[4], Object::Integer(30));
            assert_eq!(vals[5], Object::Integer(45));
        }
        other => panic!("expected array from Time components test, got {:?}", other),
    }
}

#[test]
fn test_time_day_of_week() {
    let input = r#"
        let jan1_1970 = 0;
        let dow = Time::dayOfWeek(jan1_1970);
        dow;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(4));

    let input2 = r#"
        let ts = 1718458245000;
        Time::dayOfWeek(ts);
    "#;

    let obj2 = eval_input(input2);
    assert_eq!(obj2, Object::Integer(6));
}

#[test]
fn test_time_format() {
    let input = r#"
        let ts = 1718458245000;
        Time::format(ts, "%Y-%m-%d %H:%M:%S");
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("2024-06-15 13:30:45".to_string()));
}

#[test]
fn test_time_format_partial() {
    let input = r#"
        let ts = 1718458245000;
        Time::format(ts, "%Y/%m/%d");
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("2024/06/15".to_string()));
}

#[test]
fn test_time_to_object() {
    let input = r#"
        let ts = 1718458245000;
        let obj = Time::toObject(ts);
        [obj.year, obj.month, obj.day, obj.hour, obj.minute, obj.second, obj.dayOfWeek];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 7);
            assert_eq!(vals[0], Object::Integer(2024));
            assert_eq!(vals[1], Object::Integer(6));
            assert_eq!(vals[2], Object::Integer(15));
            assert_eq!(vals[3], Object::Integer(13));
            assert_eq!(vals[4], Object::Integer(30));
            assert_eq!(vals[5], Object::Integer(45));
            assert_eq!(vals[6], Object::Integer(6));
        }
        other => panic!("expected array from Time::toObject test, got {:?}", other),
    }
}

#[test]
fn test_time_epoch() {
    let input = r#"
        let ts = 0;
        let year = Time::year(ts);
        let month = Time::month(ts);
        let day = Time::day(ts);
        let hour = Time::hour(ts);
        let minute = Time::minute(ts);
        let second = Time::second(ts);
        [year, month, day, hour, minute, second];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Integer(1970));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(1));
            assert_eq!(vals[3], Object::Integer(0));
            assert_eq!(vals[4], Object::Integer(0));
            assert_eq!(vals[5], Object::Integer(0));
        }
        other => panic!("expected array from epoch test, got {:?}", other),
    }
}

#[test]
fn test_time_error_handling() {
    let input = r#"Time::year("not a timestamp");"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from Time::year with string, got {:?}", other),
    }

    let input2 = r#"Time::format(123);"#;
    let obj2 = eval_input(input2);
    match obj2 {
        Object::Error(_) => {}
        other => panic!("expected error from Time::format with one arg, got {:?}", other),
    }

    let input3 = r#"Time::now(123);"#;
    let obj3 = eval_input(input3);
    match obj3 {
        Object::Error(_) => {}
        other => panic!("expected error from Time::now with arg, got {:?}", other),
    }

    let input4 = r#"Time::sleep(-100);"#;
    let obj4 = eval_input(input4);
    match obj4 {
        Object::Error(_) => {}
        other => panic!("expected error from Time::sleep with negative, got {:?}", other),
    }
}



