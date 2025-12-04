use crate::env::EnvRef;
use crate::object::Object;

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(false) => false,
        Object::Null => false,
        _ => true,
    }
}

pub fn test_assert(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() == 0 || args.len() > 2 {
        return Object::error("Test::assert expects 1 or 2 arguments (condition, optional message)");
    }

    let condition = &args[0];
    let message = if args.len() == 2 {
        Some(args[1].to_string())
    } else {
        None
    };

    if is_truthy(condition) {
        Object::Null
    } else {
        let base = "Assertion failed".to_string();
        let full = match message {
            Some(msg) => format!("{}: {}", base, msg),
            None => base,
        };
        Object::Error(full)
    }
}

pub fn test_assert_eq(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::error(
            "Test::assertEq expects 2 or 3 arguments (expected, actual, optional message)",
        );
    }

    let expected = &args[0];
    let actual = &args[1];
    let message = if args.len() == 3 {
        Some(args[2].to_string())
    } else {
        None
    };

    if expected == actual {
        Object::Null
    } else {
        let base = format!("Assertion failed: expected {:?}, got {:?}", expected, actual);
        let full = match message {
            Some(msg) => format!("{} - {}", base, msg),
            None => base,
        };
        Object::Error(full)
    }
}

pub fn test_assert_not_eq(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::error(
            "Test::assertNotEq expects 2 or 3 arguments (not_expected, actual, optional message)",
        );
    }

    let not_expected = &args[0];
    let actual = &args[1];
    let message = if args.len() == 3 {
        Some(args[2].to_string())
    } else {
        None
    };

    if not_expected != actual {
        Object::Null
    } else {
        let base = format!(
            "Assertion failed: values are equal but expected inequality: {:?}",
            actual
        );
        let full = match message {
            Some(msg) => format!("{} - {}", base, msg),
            None => base,
        };
        Object::Error(full)
    }
}


