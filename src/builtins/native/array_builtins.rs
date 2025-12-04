use std::rc::Rc;

use crate::env::EnvRef;
use crate::evaluator::core::expr::apply_function_with_this;
use crate::object::Object;

/// Array::map(arr, f) – returns a new array with f(element) applied to each element.
pub(crate) fn array_map(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::map expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            let mut out = Vec::with_capacity(elems.len());

            for elem in elems.into_iter() {
                let result =
                    apply_function_with_this(func.clone(), vec![elem], None, Rc::clone(&env));
                if result.is_error() {
                    return result;
                }
                out.push(result);
            }

            Object::Array(out)
        }
        other => Object::error(format!(
            "Array::map expects an Array value as first argument, got {:?}",
            other
        )),
    }
}

/// Array::filter(arr, f) – returns a new array with elements where f(element) is true.
pub(crate) fn array_filter(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::filter expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            let mut out = Vec::new();

            for elem in elems.iter() {
                let predicate = apply_function_with_this(
                    func.clone(),
                    vec![elem.clone()],
                    None,
                    Rc::clone(&env),
                );

                match predicate {
                    Object::Boolean(true) => out.push(elem.clone()),
                    Object::Boolean(false) => {}
                    other => {
                        return Object::error(format!(
                            "Array::filter predicate must return boolean, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Array(out)
        }
        other => Object::error(format!(
            "Array::filter expects an Array value as first argument, got {:?}",
            other
        )),
    }
}

/// Array::reduce(arr, initial, f) – folds array with accumulator function f(acc, element).
pub(crate) fn array_reduce(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 3 {
        return Object::error("Array::reduce expects exactly 3 arguments (array, initial, fn)");
    }

    let func = args.pop().unwrap();
    let mut acc = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            for elem in elems.into_iter() {
                let result = apply_function_with_this(
                    func.clone(),
                    vec![acc, elem],
                    None,
                    Rc::clone(&env),
                );
                if result.is_error() {
                    return result;
                }
                acc = result;
            }

            acc
        }
        other => Object::error(format!(
            "Array::reduce expects an Array value as first argument, got {:?}",
            other
        )),
    }
}


