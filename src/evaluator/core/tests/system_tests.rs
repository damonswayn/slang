use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_sys_cwd() {
    let input = r#"
        let cwd = Sys::cwd();
        Type::isString(cwd);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_platform() {
    let input = r#"
        let platform = Sys::platform();
        Type::isString(platform);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_arch() {
    let input = r#"
        let arch = Sys::arch();
        Type::isString(arch);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_args() {
    let input = r#"
        let args = Sys::args();
        Type::isArray(args);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_env_all() {
    let input = r#"
        let envVars = Sys::env();
        Type::isObject(envVars);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_env_specific() {
    let input = r#"
        let pathVar = Sys::env("PATH");
        Type::isOption(pathVar);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_env_missing() {
    let input = r#"
        let missing = Sys::env("THIS_ENV_VAR_SHOULD_NOT_EXIST_12345");
        Option::isNone(missing);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_set_env() {
    let input = r#"
        Sys::setEnv("SLANG_TEST_VAR", "hello_world");
        let val = Sys::env("SLANG_TEST_VAR");
        Option::unwrapOr(val, "default");
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("hello_world".to_string()));
}

#[test]
fn test_sys_exec() {
    let input = r#"
        let result = Sys::exec("echo hello");
        Result::isOk(result);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
fn test_sys_exec_output() {
    let input = r#"
        let result = Sys::exec("echo hello");
        let output = Result::unwrapOr(result, { code: -1, stdout: "", stderr: "" });
        output.code;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(0));
}

#[test]
fn test_sys_exec_stdout() {
    let input = r#"
        let result = Sys::exec("echo hello");
        let output = Result::unwrapOr(result, { code: -1, stdout: "", stderr: "" });
        String::trim(output.stdout);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::String("hello".to_string()));
}

#[test]
fn test_sys_error_handling() {
    let input = r#"Sys::env(123);"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from Sys::env with integer, got {:?}", other),
    }

    let input2 = r#"Sys::exec(123);"#;
    let obj2 = eval_input(input2);
    match obj2 {
        Object::Error(_) => {}
        other => panic!("expected error from Sys::exec with integer, got {:?}", other),
    }

    let input3 = r#"Sys::cwd("arg");"#;
    let obj3 = eval_input(input3);
    match obj3 {
        Object::Error(_) => {}
        other => panic!("expected error from Sys::cwd with arg, got {:?}", other),
    }
}

