use crate::env::EnvRef;
use crate::object::Object;

fn expect_one_arg(mut args: Vec<Object>, name: &str) -> Result<Object, Object> {
    if args.len() != 1 {
        return Err(Object::error(format!("{name} expects exactly 1 argument")));
    }
    Ok(args.pop().unwrap())
}

fn expect_two_args(mut args: Vec<Object>, name: &str) -> Result<(Object, Object), Object> {
    if args.len() != 2 {
        return Err(Object::error(format!(
            "{name} expects exactly 2 arguments"
        )));
    }
    let second = args.pop().unwrap();
    let first = args.pop().unwrap();
    Ok((first, second))
}

/// String::trim(s) – trims leading and trailing whitespace.
pub(crate) fn string_trim(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "String::trim") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::String(s) => Object::String(s.trim().to_string()),
        other => Object::error(format!(
            "String::trim expects a string, got {:?}",
            other
        )),
    }
}

/// String::toUpper(s) – converts to upper case.
pub(crate) fn string_to_upper(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "String::toUpper") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::String(s) => Object::String(s.to_uppercase()),
        other => Object::error(format!(
            "String::toUpper expects a string, got {:?}",
            other
        )),
    }
}

/// String::toLower(s) – converts to lower case.
pub(crate) fn string_to_lower(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "String::toLower") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::String(s) => Object::String(s.to_lowercase()),
        other => Object::error(format!(
            "String::toLower expects a string, got {:?}",
            other
        )),
    }
}

/// String::split(s, sep) – splits a string on a separator, returns Array<String>.
pub(crate) fn string_split(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, sep) = match expect_two_args(args, "String::split") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::split expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let sep_val = match sep {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::split expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let parts: Vec<Object> = if sep_val.is_empty() {
        s_val
            .chars()
            .map(|ch| Object::String(ch.to_string()))
            .collect()
    } else {
        s_val
            .split(&sep_val)
            .map(|p| Object::String(p.to_string()))
            .collect()
    };

    Object::Array(parts)
}

/// String::join(arr, sep) – joins an array of strings with a separator.
pub(crate) fn string_join(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr, sep) = match expect_two_args(args, "String::join") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let sep_val = match sep {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::join expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let elements = match arr {
        Object::Array(elems) => elems,
        other => {
            return Object::error(format!(
                "String::join expects array as first argument, got {:?}",
                other
            ))
        }
    };

    let mut out_parts = Vec::new();
    for el in elements {
        match el {
            Object::String(s) => out_parts.push(s),
            other => {
                return Object::error(format!(
                    "String::join expects array of strings, found element {:?}",
                    other
                ))
            }
        }
    }

    Object::String(out_parts.join(&sep_val))
}


