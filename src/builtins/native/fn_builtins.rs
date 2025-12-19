use std::collections::HashMap;

use crate::env::EnvRef;
use crate::evaluator::core::expr::apply_function_with_this;
use crate::object::Object;

/// Fn::identity(x) -> x
/// Returns its argument unchanged
pub(crate) fn fn_identity(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Fn::identity expects exactly 1 argument");
    }
    args.pop().unwrap()
}

/// Fn::constant(value) -> fn() -> value
/// Returns a function that always returns the given value
pub(crate) fn fn_constant(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Fn::constant expects exactly 1 argument");
    }

    let value = args.pop().unwrap();

    // Create a closure that captures the value
    // We'll return a special object that holds the constant value
    let mut captured = HashMap::new();
    captured.insert("__constant_value__".to_string(), value);
    captured.insert("__is_constant_fn__".to_string(), Object::Boolean(true));
    
    Object::Object(captured)
}

/// Fn::compose(f, g) -> fn(x) -> f(g(x))
/// Composes two functions, applying g first then f
pub(crate) fn fn_compose(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Fn::compose expects exactly 2 arguments (f, g)");
    }

    let g = args.pop().unwrap();
    let f = args.pop().unwrap();

    // Validate both are callable
    if !is_callable(&f) {
        return Object::error(format!(
            "Fn::compose first argument must be callable, got {:?}",
            f
        ));
    }
    if !is_callable(&g) {
        return Object::error(format!(
            "Fn::compose second argument must be callable, got {:?}",
            g
        ));
    }

    // Store both functions in an object
    let mut composed = HashMap::new();
    composed.insert("__compose_f__".to_string(), f);
    composed.insert("__compose_g__".to_string(), g);
    composed.insert("__is_composed__".to_string(), Object::Boolean(true));

    Object::Object(composed)
}

/// Fn::pipe(g, f) -> fn(x) -> f(g(x))
/// Pipes two functions, applying g first then f (opposite order of compose)
pub(crate) fn fn_pipe(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Fn::pipe expects exactly 2 arguments (g, f)");
    }

    let f = args.pop().unwrap();
    let g = args.pop().unwrap();

    // Validate both are callable
    if !is_callable(&g) {
        return Object::error(format!(
            "Fn::pipe first argument must be callable, got {:?}",
            g
        ));
    }
    if !is_callable(&f) {
        return Object::error(format!(
            "Fn::pipe second argument must be callable, got {:?}",
            f
        ));
    }

    // Store both functions in an object (same as compose but reversed)
    let mut piped = HashMap::new();
    piped.insert("__compose_f__".to_string(), f);
    piped.insert("__compose_g__".to_string(), g);
    piped.insert("__is_composed__".to_string(), Object::Boolean(true));

    Object::Object(piped)
}

/// Fn::apply(f, args) -> f(...args)
/// Applies a function to an array of arguments
pub(crate) fn fn_apply(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Fn::apply expects exactly 2 arguments (function, args_array)");
    }

    let fn_args = args.pop().unwrap();
    let func = args.pop().unwrap();

    let mut args_vec = match fn_args {
        Object::Array(arr) => arr,
        other => {
            return Object::error(format!(
                "Fn::apply second argument must be an array, got {:?}",
                other
            ))
        }
    };

    // Handle special function objects
    if let Object::Object(ref map) = func {
        // Handle composed functions
        if map.get("__is_composed__") == Some(&Object::Boolean(true)) {
            if let (Some(f), Some(g)) = (map.get("__compose_f__"), map.get("__compose_g__")) {
                let g_result = apply_function_with_this(g.clone(), args_vec, None, env.clone());
                if matches!(g_result, Object::Error(_)) {
                    return g_result;
                }
                return apply_function_with_this(f.clone(), vec![g_result], None, env);
            }
        }

        // Handle constant functions
        if map.get("__is_constant_fn__") == Some(&Object::Boolean(true)) {
            if let Some(value) = map.get("__constant_value__") {
                return value.clone();
            }
        }

        // Handle negated functions
        if map.get("__is_negated__") == Some(&Object::Boolean(true)) {
            if let Some(pred) = map.get("__negated_fn__") {
                let result = apply_function_with_this(pred.clone(), args_vec, None, env);
                return match result {
                    Object::Boolean(b) => Object::Boolean(!b),
                    Object::Error(_) => result,
                    other => Object::error(format!(
                        "Negated function must return boolean, got {:?}",
                        other
                    )),
                };
            }
        }

        // Handle flipped functions
        if map.get("__is_flipped__") == Some(&Object::Boolean(true)) {
            if let Some(inner_func) = map.get("__flipped_fn__") {
                if args_vec.len() >= 2 {
                    args_vec.swap(0, 1);
                }
                return apply_function_with_this(inner_func.clone(), args_vec, None, env);
            }
        }

        // Handle partial application
        if map.get("__is_partial__") == Some(&Object::Boolean(true)) {
            if let (Some(inner_func), Some(Object::Array(bound))) =
                (map.get("__partial_fn__"), map.get("__partial_args__"))
            {
                let mut all_args = bound.clone();
                all_args.extend(args_vec);
                return apply_function_with_this(inner_func.clone(), all_args, None, env);
            }
        }
    }

    if !is_callable(&func) {
        return Object::error(format!(
            "Fn::apply first argument must be callable, got {:?}",
            func
        ));
    }

    apply_function_with_this(func, args_vec, None, env)
}

/// Fn::call(f, ...args) -> f(...args)
/// Calls a function with the provided arguments
pub(crate) fn fn_call(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.is_empty() {
        return Object::error("Fn::call expects at least 1 argument (function)");
    }

    let func = args.remove(0);

    // Handle special function objects
    if let Object::Object(ref map) = func {
        // Handle composed functions
        if map.get("__is_composed__") == Some(&Object::Boolean(true)) {
            if let (Some(f), Some(g)) = (map.get("__compose_f__"), map.get("__compose_g__")) {
                let g_result = apply_function_with_this(g.clone(), args, None, env.clone());
                if matches!(g_result, Object::Error(_)) {
                    return g_result;
                }
                return apply_function_with_this(f.clone(), vec![g_result], None, env);
            }
        }

        // Handle constant functions
        if map.get("__is_constant_fn__") == Some(&Object::Boolean(true)) {
            if let Some(value) = map.get("__constant_value__") {
                return value.clone();
            }
        }

        // Handle negated functions
        if map.get("__is_negated__") == Some(&Object::Boolean(true)) {
            if let Some(pred) = map.get("__negated_fn__") {
                let result = apply_function_with_this(pred.clone(), args, None, env);
                return match result {
                    Object::Boolean(b) => Object::Boolean(!b),
                    Object::Error(_) => result,
                    other => Object::error(format!(
                        "Negated function must return boolean, got {:?}",
                        other
                    )),
                };
            }
        }

        // Handle flipped functions
        if map.get("__is_flipped__") == Some(&Object::Boolean(true)) {
            if let Some(inner_func) = map.get("__flipped_fn__") {
                if args.len() >= 2 {
                    args.swap(0, 1);
                }
                return apply_function_with_this(inner_func.clone(), args, None, env);
            }
        }

        // Handle partial application
        if map.get("__is_partial__") == Some(&Object::Boolean(true)) {
            if let (Some(inner_func), Some(Object::Array(bound))) =
                (map.get("__partial_fn__"), map.get("__partial_args__"))
            {
                let mut all_args = bound.clone();
                all_args.extend(args);
                return apply_function_with_this(inner_func.clone(), all_args, None, env);
            }
        }
    }

    if !is_callable(&func) {
        return Object::error(format!(
            "Fn::call first argument must be callable, got {:?}",
            func
        ));
    }

    apply_function_with_this(func, args, None, env)
}

/// Fn::negate(predicate) -> fn(x) -> !predicate(x)
/// Returns a function that negates the result of the predicate
pub(crate) fn fn_negate(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Fn::negate expects exactly 1 argument (predicate)");
    }

    let pred = args.pop().unwrap();

    if !is_callable(&pred) {
        return Object::error(format!(
            "Fn::negate argument must be callable, got {:?}",
            pred
        ));
    }

    let mut negated = HashMap::new();
    negated.insert("__negated_fn__".to_string(), pred);
    negated.insert("__is_negated__".to_string(), Object::Boolean(true));

    Object::Object(negated)
}

/// Fn::flip(f) -> fn(a, b) -> f(b, a)
/// Returns a function with the first two arguments flipped
pub(crate) fn fn_flip(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Fn::flip expects exactly 1 argument (function)");
    }

    let func = args.pop().unwrap();

    if !is_callable(&func) {
        return Object::error(format!(
            "Fn::flip argument must be callable, got {:?}",
            func
        ));
    }

    let mut flipped = HashMap::new();
    flipped.insert("__flipped_fn__".to_string(), func);
    flipped.insert("__is_flipped__".to_string(), Object::Boolean(true));

    Object::Object(flipped)
}

/// Fn::partial(f, ...boundArgs) -> fn(...remainingArgs) -> f(...boundArgs, ...remainingArgs)
/// Partially applies arguments to a function
pub(crate) fn fn_partial(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.is_empty() {
        return Object::error("Fn::partial expects at least 1 argument (function)");
    }

    let func = args.remove(0);
    let bound_args = args;

    if !is_callable(&func) {
        return Object::error(format!(
            "Fn::partial first argument must be callable, got {:?}",
            func
        ));
    }

    let mut partial = HashMap::new();
    partial.insert("__partial_fn__".to_string(), func);
    partial.insert("__partial_args__".to_string(), Object::Array(bound_args));
    partial.insert("__is_partial__".to_string(), Object::Boolean(true));

    Object::Object(partial)
}

/// Fn::isCallable(value) -> boolean
/// Returns true if the value is a function or builtin
pub(crate) fn fn_is_callable(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Fn::isCallable expects exactly 1 argument");
    }

    let value = args.pop().unwrap();
    Object::Boolean(is_callable(&value))
}

fn is_callable(obj: &Object) -> bool {
    matches!(obj, Object::Function { .. } | Object::Builtin(_))
}
