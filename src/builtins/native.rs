use crate::object::Object;
use crate::object::Object::Integer;
use crate::object::types::BuiltinFunction;
use crate::env::EnvRef;
use crate::evaluator::core::expr::apply_function_with_this;
use regex::Regex;

mod file_builtins;

pub struct Builtin {
    pub name: &'static str,
    pub func: BuiltinFunction,
}

const BUILTINS: &[Builtin] = &[
    Builtin { name: "len",   func: builtin_len },
    Builtin { name: "first", func: builtin_first },
    Builtin { name: "last",  func: builtin_last },
    Builtin { name: "rest",  func: builtin_rest },
    Builtin { name: "push",  func: builtin_push },
    Builtin { name: "print", func: builtin_print },
    Builtin { name: "debug", func: builtin_debug },

    // Regex builtins
    Builtin { name: "regexIsMatch", func: builtin_regex_is_match },
    Builtin { name: "regexFind", func: builtin_regex_find },
    Builtin { name: "regexReplace", func: builtin_regex_replace },
    Builtin { name: "regexMatch", func: builtin_regex_match },

    // File builtins
    Builtin { name: "file_open", func: file_builtins::builtin_open },
    Builtin { name: "file_read", func: file_builtins::builtin_read },
    Builtin { name: "file_write", func: file_builtins::builtin_write },
    Builtin { name: "file_seek", func: file_builtins::builtin_seek },
    Builtin { name: "file_close", func: file_builtins::builtin_close },
];

pub fn get(name: &str) -> Option<BuiltinFunction> {
    for b in BUILTINS {
        if b.name == name {
            return Some(b.func);
        }
    }
    None
}

// Native func implementations

fn builtin_len(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("len expects exactly 1 argument");
    }

    match &args[0] {
        Object::String(s) => Integer(s.len() as i64),
        Object::Array(a) => Integer(a.len() as i64),
        other => Object::error(format!("len not supported for value: {:?}", other)),
    }
}

fn builtin_first(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("first expects exactly 1 argument");
    }

    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() {
                Object::Null
            } else {
                elems[0].clone()
            }
        }
        other => Object::error(format!("first expects array, got {:?}", other)),
    }
}

fn builtin_last(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("last expects exactly 1 argument");
    }

    match &args[0] {
        Object::Array(elems) => {
            if let Some(last) = elems.last() {
                last.clone()
            } else {
                Object::Null
            }
        }
        other => Object::error(format!("last expects array, got {:?}", other)),
    }
}

fn builtin_rest(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("rest expects exactly 1 argument");
    }

    match &args[0] {
        Object::Array(elems) => {
            if elems.len() <= 1 {
                Object::Array(vec![])
            } else {
                Object::Array(elems[1..].to_vec())
            }
        }
        other => Object::error(format!("rest expects array, got {:?}", other)),
    }
}

fn builtin_push(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("push expects exactly 2 arguments");
    }

    let value = args.pop().unwrap();
    let array = args.pop().unwrap();

    match array {
        Object::Array(mut elems) => {
            elems.push(value);
            Object::Array(elems)
        }
        other => Object::error(format!("push expects array as first argument, got {:?}", other)),
    }
}

fn builtin_print(args: Vec<Object>, _env: EnvRef) -> Object {
    // Just print all args separated by space and newline
    let text = args
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", text);
    Object::Null
}

fn builtin_debug(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("debug expects exactly 1 argument");
    }

    match &args[0] {
        Object::Boolean(true) => {
            crate::debug::enable_debug_mode();
            Object::Boolean(true)
        },
        Object::Boolean(false) => {
            crate::debug::disable_debug_mode();
            Object::Boolean(false)
        },
        other => Object::error(format!("debug expects boolean, got {:?}", other)),
    }
}

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

// ----- Option / Result monad helpers -----
//
// These are not exposed as top-level builtins by name; instead they are
// attached to the pre-bound `Option` and `Result` objects in the global
// environment, so user code calls:
//
//   Option::Some(value)
//   Option::None()
//   Option::isSome(opt)
//   Option::isNone(opt)
//   Option::unwrapOr(opt, default)
//
//   Option::map(opt, f)
//   Option::andThen(opt, f) / Option::bind(opt, f)
//
//   Result::Ok(value)
//   Result::Err("msg")
//   Result::isOk(res)
//   Result::isErr(res)
//   Result::unwrapOr(res, default)
//   Result::map(res, f)
//   Result::andThen(res, f) / Result::bind(res, f)
//

pub(crate) fn option_some(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Option::Some expects exactly 1 argument");
    }

    let value = args.into_iter().next().unwrap();
    Object::OptionSome(Box::new(value))
}

pub(crate) fn option_none(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Option::None expects no arguments");
    }

    Object::OptionNone
}

pub(crate) fn option_is_some(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Option::isSome expects exactly 1 argument (an Option)");
    }

    match &args[0] {
        Object::OptionSome(_) => Object::Boolean(true),
        Object::OptionNone => Object::Boolean(false),
        other => Object::error(format!(
            "Option::isSome expects an Option value, got {:?}",
            other
        )),
    }
}

pub(crate) fn option_is_none(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Option::isNone expects exactly 1 argument (an Option)");
    }

    match &args[0] {
        Object::OptionSome(_) => Object::Boolean(false),
        Object::OptionNone => Object::Boolean(true),
        other => Object::error(format!(
            "Option::isNone expects an Option value, got {:?}",
            other
        )),
    }
}

pub(crate) fn option_unwrap_or(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Option::unwrapOr expects exactly 2 arguments (option, default)");
    }

    let default = args.pop().unwrap();
    let opt = args.pop().unwrap();

    match opt {
        Object::OptionSome(inner) => *inner,
        Object::OptionNone => default,
        other => Object::error(format!(
            "Option::unwrapOr expects an Option value as first argument, got {:?}",
            other
        )),
    }
}

/// Option::map(opt, f) – if opt is Some(v), returns Some(f(v)); if None, returns None.
pub(crate) fn option_map(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Option::map expects exactly 2 arguments (option, fn)");
    }

    let func = args.pop().unwrap();
    let opt = args.pop().unwrap();

    match opt {
        Object::OptionSome(inner) => {
            let result = apply_function_with_this(func, vec![*inner], None, env);
            if result.is_error() {
                result
            } else {
                Object::OptionSome(Box::new(result))
            }
        }
        Object::OptionNone => Object::OptionNone,
        other => Object::error(format!(
            "Option::map expects an Option value as first argument, got {:?}",
            other
        )),
    }
}

/// Option::andThen(opt, f) – monadic bind. If opt is Some(v), returns f(v)
/// (which should itself return an Option); if None, returns None.
pub(crate) fn option_and_then(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Option::andThen expects exactly 2 arguments (option, fn)");
    }

    let func = args.pop().unwrap();
    let opt = args.pop().unwrap();

    match opt {
        Object::OptionSome(inner) => {
            apply_function_with_this(func, vec![*inner], None, env)
        }
        Object::OptionNone => Object::OptionNone,
        other => Object::error(format!(
            "Option::andThen expects an Option value as first argument, got {:?}",
            other
        )),
    }
}

/// Alias: Option::bind = Option::and_then
pub(crate) fn option_bind(args: Vec<Object>, env: EnvRef) -> Object {
    option_and_then(args, env)
}

/// Alias: Option::fmap = Option::map
pub(crate) fn option_fmap(args: Vec<Object>, env: EnvRef) -> Object {
    option_map(args, env)
}

pub(crate) fn result_ok(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Result::Ok expects exactly 1 argument");
    }

    let value = args.into_iter().next().unwrap();
    Object::ResultOk(Box::new(value))
}

pub(crate) fn result_err(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Result::Err expects exactly 1 argument (an error value)");
    }

    let value = args.into_iter().next().unwrap();
    Object::ResultErr(Box::new(value))
}

pub(crate) fn result_is_ok(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Result::isOk expects exactly 1 argument (a Result)");
    }

    match &args[0] {
        Object::ResultOk(_) => Object::Boolean(true),
        Object::ResultErr(_) => Object::Boolean(false),
        other => Object::error(format!(
            "Result::isOk expects a Result value, got {:?}",
            other
        )),
    }
}

pub(crate) fn result_is_err(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Result::isErr expects exactly 1 argument (a Result)");
    }

    match &args[0] {
        Object::ResultOk(_) => Object::Boolean(false),
        Object::ResultErr(_) => Object::Boolean(true),
        other => Object::error(format!(
            "Result::isErr expects a Result value, got {:?}",
            other
        )),
    }
}

pub(crate) fn result_unwrap_or(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Result::unwrapOr expects exactly 2 arguments (result, default)");
    }

    let default = args.pop().unwrap();
    let res = args.pop().unwrap();

    match res {
        Object::ResultOk(inner) => *inner,
        Object::ResultErr(_) => default,
        other => Object::error(format!(
            "Result::unwrapOr expects a Result value as first argument, got {:?}",
            other
        )),
    }
}

/// Result::map(res, f) – if Ok(v), returns Ok(f(v)); if Err(e), returns Err(e).
pub(crate) fn result_map(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Result::map expects exactly 2 arguments (result, fn)");
    }

    let func = args.pop().unwrap();
    let res = args.pop().unwrap();

    match res {
        Object::ResultOk(inner) => {
            let result = apply_function_with_this(func, vec![*inner], None, env);
            if result.is_error() {
                result
            } else {
                Object::ResultOk(Box::new(result))
            }
        }
        Object::ResultErr(err) => Object::ResultErr(err),
        other => Object::error(format!(
            "Result::map expects a Result value as first argument, got {:?}",
            other
        )),
    }
}

/// Result::andThen(res, f) – monadic bind. If Ok(v), returns f(v)
/// (which should itself return a Result); if Err(e), returns Err(e).
pub(crate) fn result_and_then(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Result::andThen expects exactly 2 arguments (result, fn)");
    }

    let func = args.pop().unwrap();
    let res = args.pop().unwrap();

    match res {
        Object::ResultOk(inner) => {
            apply_function_with_this(func, vec![*inner], None, env)
        }
        Object::ResultErr(err) => Object::ResultErr(err),
        other => Object::error(format!(
            "Result::andThen expects a Result value as first argument, got {:?}",
            other
        )),
    }
}

/// Alias: Result::bind = Result::and_then
pub(crate) fn result_bind(args: Vec<Object>, env: EnvRef) -> Object {
    result_and_then(args, env)
}

/// Alias: Result::fmap = Result::map
pub(crate) fn result_fmap(args: Vec<Object>, env: EnvRef) -> Object {
    result_map(args, env)
}