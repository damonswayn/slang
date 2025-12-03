use crate::env::EnvRef;
use crate::object::Object;
use regex::Regex;

// ----- Regex builtins -----
//
// Simple, one-shot regex helpers backed by Rust's `regex` crate.
//
//   regexIsMatch(text, pattern) -> Boolean
//   regexFind(text, pattern)    -> Option::Some(matched_string) | Option::None()
//   regexReplace(text, pattern, replacement) -> String
//   regexMatch(text, pattern)   -> Option::Some([full, g1, g2, ...]) | Option::None()
//
// These are also exposed under the `Regex` namespace for consistency:
//
//   Regex::isMatch(text, pattern)
//   Regex::find(text, pattern)
//   Regex::replace(text, pattern, replacement)
//   Regex::match(text, pattern)
//

pub(crate) fn builtin_regex_is_match(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("regexIsMatch expects exactly 2 arguments (text, pattern)");
    }

    let text = match &args[0] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexIsMatch expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let pattern = match &args[1] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexIsMatch expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Object::error(format!("invalid regex pattern: {}", e)),
    };

    Object::Boolean(re.is_match(text))
}

pub(crate) fn builtin_regex_find(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("regexFind expects exactly 2 arguments (text, pattern)");
    }

    let text = match &args[0] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexFind expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let pattern = match &args[1] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexFind expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Object::error(format!("invalid regex pattern: {}", e)),
    };

    if let Some(m) = re.find(text) {
        Object::OptionSome(Box::new(Object::String(m.as_str().to_string())))
    } else {
        Object::OptionNone
    }
}

pub(crate) fn builtin_regex_replace(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 3 {
        return Object::error("regexReplace expects exactly 3 arguments (text, pattern, replacement)");
    }

    let text = match &args[0] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexReplace expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let pattern = match &args[1] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexReplace expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let replacement = match &args[2] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexReplace expects string as third argument, got {:?}",
                other
            ))
        }
    };

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Object::error(format!("invalid regex pattern: {}", e)),
    };

    let result = re.replace_all(text, replacement.as_str());
    Object::String(result.to_string())
}

pub(crate) fn builtin_regex_match(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("regexMatch expects exactly 2 arguments (text, pattern)");
    }

    let text = match &args[0] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexMatch expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let pattern = match &args[1] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "regexMatch expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Object::error(format!("invalid regex pattern: {}", e)),
    };

    match re.captures(text) {
        Some(caps) => {
            let mut groups = Vec::with_capacity(caps.len());
            for i in 0..caps.len() {
                if let Some(m) = caps.get(i) {
                    groups.push(Object::String(m.as_str().to_string()));
                } else {
                    groups.push(Object::Null);
                }
            }
            Object::OptionSome(Box::new(Object::Array(groups)))
        }
        None => Object::OptionNone,
    }
}


