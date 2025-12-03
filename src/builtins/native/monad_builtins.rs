use crate::env::EnvRef;
use crate::evaluator::core::expr::apply_function_with_this;
use crate::object::Object;

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


