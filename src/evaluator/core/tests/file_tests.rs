use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_file_namespace_result_helpers() {
    let input = r#"
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
            assert_eq!(vals[0], Object::Boolean(false));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(false));
            assert_eq!(vals[3], Object::Boolean(true));
        }
        other => panic!("expected array from file open error test, got {:?}", other),
    }
}

#[test]
fn test_file_namespace_read_write_errors() {
    let input = r#"
        let opened = File::open("tmp_file_namespace_errors.txt", "w+");
        let f = Result::unwrapOr(opened, 0);

        let res1 = File::read(123);
        let res2 = File::write(123, "data");

        let res3 = File::write(f, 42);

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
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(true));
        }
        other => panic!("expected array from file read/write error test, got {:?}", other),
    }
}



