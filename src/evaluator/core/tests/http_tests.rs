use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_http_get_error_handling() {
    let input = r#"HTTP::get(123);"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::get with integer, got {:?}", other),
    }

    let input2 = r#"HTTP::get();"#;
    let obj2 = eval_input(input2);
    match obj2 {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::get with no args, got {:?}", other),
    }
}

#[test]
fn test_http_post_error_handling() {
    let input = r#"HTTP::post(123, "body");"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::post with integer URL, got {:?}", other),
    }

    let input2 = r#"HTTP::post("http://example.com");"#;
    let obj2 = eval_input(input2);
    match obj2 {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::post with 1 arg, got {:?}", other),
    }
}

#[test]
fn test_http_put_error_handling() {
    let input = r#"HTTP::put(123, "body");"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::put with integer URL, got {:?}", other),
    }
}

#[test]
fn test_http_delete_error_handling() {
    let input = r#"HTTP::delete(123);"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::delete with integer, got {:?}", other),
    }
}

#[test]
fn test_http_patch_error_handling() {
    let input = r#"HTTP::patch(123, "body");"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::patch with integer URL, got {:?}", other),
    }
}

#[test]
fn test_http_head_error_handling() {
    let input = r#"HTTP::head(123);"#;
    let obj = eval_input(input);
    match obj {
        Object::Error(_) => {}
        other => panic!("expected error from HTTP::head with integer, got {:?}", other),
    }
}

// Network tests - these require actual network access
// Run with: cargo test -- --ignored

#[test]
#[ignore]
fn test_http_get_real() {
    let input = r#"
        let result = HTTP::get("https://httpbin.org/get");
        Result::isOk(result);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
#[ignore]
fn test_http_get_response_structure() {
    let input = r#"
        let result = HTTP::get("https://httpbin.org/get");
        let response = Result::unwrapOr(result, { status: 0, body: "" });
        response.status;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(200));
}

#[test]
#[ignore]
fn test_http_post_real() {
    let input = r#"
        let result = HTTP::post("https://httpbin.org/post", "test body");
        Result::isOk(result);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
#[ignore]
fn test_http_post_json() {
    let input = r#"
        let result = HTTP::post("https://httpbin.org/post", { name: "test", value: 42 });
        let response = Result::unwrapOr(result, { status: 0 });
        response.status;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Integer(200));
}

#[test]
#[ignore]
fn test_http_with_headers() {
    let input = r#"
        let result = HTTP::get("https://httpbin.org/headers", {
            headers: { "X-Custom-Header": "test-value" }
        });
        Result::isOk(result);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}

#[test]
#[ignore]
fn test_http_404_error() {
    let input = r#"
        let result = HTTP::get("https://httpbin.org/status/404");
        Result::isErr(result);
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Boolean(true));
}




