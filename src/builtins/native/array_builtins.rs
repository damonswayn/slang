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

/// Array::find(arr, f) – returns Option::Some(element) of the first match, or Option::None().
pub(crate) fn array_find(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::find expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            for elem in elems.iter() {
                let predicate = apply_function_with_this(
                    func.clone(),
                    vec![elem.clone()],
                    None,
                    Rc::clone(&env),
                );

                match predicate {
                    Object::Boolean(true) => {
                        return Object::OptionSome(Box::new(elem.clone()));
                    }
                    Object::Boolean(false) => {}
                    other => {
                        return Object::error(format!(
                            "Array::find predicate must return boolean, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::OptionNone
        }
        other => Object::error(format!(
            "Array::find expects an Array value as first argument, got {:?}",
            other
        )),
    }
}

/// Array::some(arr, f) – returns true if any element matches the predicate.
pub(crate) fn array_some(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::some expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            for elem in elems.iter() {
                let predicate = apply_function_with_this(
                    func.clone(),
                    vec![elem.clone()],
                    None,
                    Rc::clone(&env),
                );

                match predicate {
                    Object::Boolean(true) => return Object::Boolean(true),
                    Object::Boolean(false) => {}
                    other => {
                        return Object::error(format!(
                            "Array::some predicate must return boolean, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Boolean(false)
        }
        other => Object::error(format!(
            "Array::some expects an Array value as first argument, got {:?}",
            other
        )),
    }
}

/// Array::every(arr, f) – returns true if all elements match the predicate.
pub(crate) fn array_every(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::every expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            for elem in elems.iter() {
                let predicate = apply_function_with_this(
                    func.clone(),
                    vec![elem.clone()],
                    None,
                    Rc::clone(&env),
                );

                match predicate {
                    Object::Boolean(true) => {}
                    Object::Boolean(false) => return Object::Boolean(false),
                    other => {
                        return Object::error(format!(
                            "Array::every predicate must return boolean, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Boolean(true)
        }
        other => Object::error(format!(
            "Array::every expects an Array value as first argument, got {:?}",
            other
        )),
    }
}

/// Array::flatMap(arr, f) – maps each element to an array and concatenates the results.
pub(crate) fn array_flat_map(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::flatMap expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            let mut out = Vec::new();

            for elem in elems.into_iter() {
                let result =
                    apply_function_with_this(func.clone(), vec![elem], None, Rc::clone(&env));

                match result {
                    Object::Array(inner) => out.extend(inner.into_iter()),
                    other => {
                        return Object::error(format!(
                            "Array::flatMap expects function to return array, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Array(out)
        }
        other => Object::error(format!(
            "Array::flatMap expects an Array value as first argument, got {:?}",
            other
        )),
    }
}

// Helper functions for argument extraction
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

fn expect_three_args(mut args: Vec<Object>, name: &str) -> Result<(Object, Object, Object), Object> {
    if args.len() != 3 {
        return Err(Object::error(format!(
            "{name} expects exactly 3 arguments"
        )));
    }
    let third = args.pop().unwrap();
    let second = args.pop().unwrap();
    let first = args.pop().unwrap();
    Ok((first, second, third))
}

/// Helper to compare Objects for sorting
fn compare_objects(a: &Object, b: &Object) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    match (a, b) {
        (Object::Integer(x), Object::Integer(y)) => x.cmp(y),
        (Object::Float(x), Object::Float(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Object::Integer(x), Object::Float(y)) => (*x as f64).partial_cmp(y).unwrap_or(Ordering::Equal),
        (Object::Float(x), Object::Integer(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(Ordering::Equal),
        (Object::String(x), Object::String(y)) => x.cmp(y),
        (Object::Boolean(x), Object::Boolean(y)) => x.cmp(y),
        // For other types, compare by debug representation
        _ => format!("{:?}", a).cmp(&format!("{:?}", b)),
    }
}

/// Array::sort(arr) – returns a new sorted array (natural ordering).
pub(crate) fn array_sort(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::sort") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(mut elems) => {
            elems.sort_by(compare_objects);
            Object::Array(elems)
        }
        other => Object::error(format!(
            "Array::sort expects an array, got {:?}",
            other
        )),
    }
}

/// Array::sortBy(arr, f) – returns a new array sorted by comparator function.
/// The function f(a, b) should return a negative number if a < b, 0 if equal, positive if a > b.
pub(crate) fn array_sort_by(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::sortBy expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(mut elems) => {
            // We need to handle errors during sorting
            let mut sort_error: Option<Object> = None;

            elems.sort_by(|a, b| {
                if sort_error.is_some() {
                    return std::cmp::Ordering::Equal;
                }

                let result = apply_function_with_this(
                    func.clone(),
                    vec![a.clone(), b.clone()],
                    None,
                    Rc::clone(&env),
                );

                match result {
                    Object::Integer(n) => {
                        if n < 0 {
                            std::cmp::Ordering::Less
                        } else if n > 0 {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }
                    Object::Error(_) => {
                        sort_error = Some(result);
                        std::cmp::Ordering::Equal
                    }
                    other => {
                        sort_error = Some(Object::error(format!(
                            "Array::sortBy comparator must return integer, got {:?}",
                            other
                        )));
                        std::cmp::Ordering::Equal
                    }
                }
            });

            if let Some(err) = sort_error {
                return err;
            }

            Object::Array(elems)
        }
        other => Object::error(format!(
            "Array::sortBy expects an array as first argument, got {:?}",
            other
        )),
    }
}

/// Array::reverse(arr) – returns a new array with elements in reverse order.
pub(crate) fn array_reverse(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::reverse") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(mut elems) => {
            elems.reverse();
            Object::Array(elems)
        }
        other => Object::error(format!(
            "Array::reverse expects an array, got {:?}",
            other
        )),
    }
}

/// Array::indexOf(arr, elem) – returns Option::Some(index) or Option::None.
pub(crate) fn array_index_of(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr, elem) = match expect_two_args(args, "Array::indexOf") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => {
            for (i, e) in elems.iter().enumerate() {
                if e == &elem {
                    return Object::OptionSome(Box::new(Object::Integer(i as i64)));
                }
            }
            Object::OptionNone
        }
        other => Object::error(format!(
            "Array::indexOf expects an array as first argument, got {:?}",
            other
        )),
    }
}

/// Array::includes(arr, elem) – returns true if array contains the element.
pub(crate) fn array_includes(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr, elem) = match expect_two_args(args, "Array::includes") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => {
            Object::Boolean(elems.contains(&elem))
        }
        other => Object::error(format!(
            "Array::includes expects an array as first argument, got {:?}",
            other
        )),
    }
}

/// Array::concat(arr1, arr2) – returns a new array with elements from both arrays.
pub(crate) fn array_concat(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr1, arr2) = match expect_two_args(args, "Array::concat") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let elems1 = match arr1 {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::concat expects an array as first argument, got {:?}",
                other
            ))
        }
    };

    let elems2 = match arr2 {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::concat expects an array as second argument, got {:?}",
                other
            ))
        }
    };

    let mut result = elems1;
    result.extend(elems2);
    Object::Array(result)
}

/// Array::slice(arr, start, end) – returns a sub-array from start (inclusive) to end (exclusive).
/// Negative indices count from the end.
pub(crate) fn array_slice(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr, start, end) = match expect_three_args(args, "Array::slice") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let elems = match arr {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::slice expects an array as first argument, got {:?}",
                other
            ))
        }
    };

    let start_val = match start {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::slice expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    let end_val = match end {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::slice expects integer as third argument, got {:?}",
                other
            ))
        }
    };

    let len = elems.len() as i64;

    // Handle negative indices
    let start_idx = if start_val < 0 {
        (len + start_val).max(0) as usize
    } else {
        start_val.min(len) as usize
    };

    let end_idx = if end_val < 0 {
        (len + end_val).max(0) as usize
    } else {
        end_val.min(len) as usize
    };

    if start_idx >= end_idx {
        return Object::Array(vec![]);
    }

    Object::Array(elems[start_idx..end_idx].to_vec())
}

/// Array::take(arr, n) – returns first n elements.
pub(crate) fn array_take(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr, n) = match expect_two_args(args, "Array::take") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let elems = match arr {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::take expects an array as first argument, got {:?}",
                other
            ))
        }
    };

    let n_val = match n {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::take expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    if n_val < 0 {
        return Object::error("Array::take count must be non-negative");
    }

    let take_count = (n_val as usize).min(elems.len());
    Object::Array(elems[..take_count].to_vec())
}

/// Array::drop(arr, n) – returns array without first n elements.
pub(crate) fn array_drop(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr, n) = match expect_two_args(args, "Array::drop") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let elems = match arr {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::drop expects an array as first argument, got {:?}",
                other
            ))
        }
    };

    let n_val = match n {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::drop expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    if n_val < 0 {
        return Object::error("Array::drop count must be non-negative");
    }

    let drop_count = (n_val as usize).min(elems.len());
    Object::Array(elems[drop_count..].to_vec())
}

/// Array::range(start, end) – generates an array of integers from start (inclusive) to end (exclusive).
/// Optional step parameter: Array::range(start, end, step)
pub(crate) fn array_range(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::error("Array::range expects 2 or 3 arguments (start, end, [step])");
    }

    let step = if args.len() == 3 {
        match args.pop().unwrap() {
            Object::Integer(i) => i,
            other => {
                return Object::error(format!(
                    "Array::range expects integer step, got {:?}",
                    other
                ))
            }
        }
    } else {
        1
    };

    let end = match args.pop().unwrap() {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::range expects integer end, got {:?}",
                other
            ))
        }
    };

    let start = match args.pop().unwrap() {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::range expects integer start, got {:?}",
                other
            ))
        }
    };

    if step == 0 {
        return Object::error("Array::range step cannot be zero");
    }

    let mut result = Vec::new();

    if step > 0 {
        let mut i = start;
        while i < end {
            result.push(Object::Integer(i));
            i += step;
        }
    } else {
        let mut i = start;
        while i > end {
            result.push(Object::Integer(i));
            i += step;
        }
    }

    Object::Array(result)
}

/// Array::unique(arr) – returns a new array with duplicate elements removed (preserves first occurrence).
pub(crate) fn array_unique(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::unique") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => {
            let mut seen = Vec::new();
            let mut result = Vec::new();

            for elem in elems {
                if !seen.contains(&elem) {
                    seen.push(elem.clone());
                    result.push(elem);
                }
            }

            Object::Array(result)
        }
        other => Object::error(format!(
            "Array::unique expects an array, got {:?}",
            other
        )),
    }
}

/// Array::flatten(arr) – flattens one level of nesting.
pub(crate) fn array_flatten(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::flatten") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => {
            let mut result = Vec::new();

            for elem in elems {
                match elem {
                    Object::Array(inner) => result.extend(inner),
                    other => result.push(other),
                }
            }

            Object::Array(result)
        }
        other => Object::error(format!(
            "Array::flatten expects an array, got {:?}",
            other
        )),
    }
}

/// Array::zip(arr1, arr2) – returns array of [a, b] pairs.
pub(crate) fn array_zip(args: Vec<Object>, _env: EnvRef) -> Object {
    let (arr1, arr2) = match expect_two_args(args, "Array::zip") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let elems1 = match arr1 {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::zip expects an array as first argument, got {:?}",
                other
            ))
        }
    };

    let elems2 = match arr2 {
        Object::Array(e) => e,
        other => {
            return Object::error(format!(
                "Array::zip expects an array as second argument, got {:?}",
                other
            ))
        }
    };

    let result: Vec<Object> = elems1
        .into_iter()
        .zip(elems2.into_iter())
        .map(|(a, b)| Object::Array(vec![a, b]))
        .collect();

    Object::Array(result)
}

/// Array::unzip(arr) – converts [[a,b], ...] to [[a,...], [b,...]].
pub(crate) fn array_unzip(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::unzip") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => {
            let mut firsts = Vec::new();
            let mut seconds = Vec::new();

            for elem in elems {
                match elem {
                    Object::Array(pair) if pair.len() >= 2 => {
                        firsts.push(pair[0].clone());
                        seconds.push(pair[1].clone());
                    }
                    other => {
                        return Object::error(format!(
                            "Array::unzip expects array of pairs, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Array(vec![Object::Array(firsts), Object::Array(seconds)])
        }
        other => Object::error(format!(
            "Array::unzip expects an array, got {:?}",
            other
        )),
    }
}

/// Array::groupBy(arr, f) – groups elements by key returned by f.
/// Returns an object { key: [elements] }.
pub(crate) fn array_group_by(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::groupBy expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            let mut groups: std::collections::HashMap<String, Vec<Object>> =
                std::collections::HashMap::new();

            for elem in elems {
                let key_result = apply_function_with_this(
                    func.clone(),
                    vec![elem.clone()],
                    None,
                    Rc::clone(&env),
                );

                let key = match key_result {
                    Object::String(s) => s,
                    Object::Integer(i) => i.to_string(),
                    Object::Boolean(b) => b.to_string(),
                    Object::Error(_) => return key_result,
                    other => {
                        return Object::error(format!(
                            "Array::groupBy key function must return string/int/bool, got {:?}",
                            other
                        ))
                    }
                };

                groups.entry(key).or_default().push(elem);
            }

            let result: std::collections::HashMap<String, Object> = groups
                .into_iter()
                .map(|(k, v)| (k, Object::Array(v)))
                .collect();

            Object::Object(result)
        }
        other => Object::error(format!(
            "Array::groupBy expects an array as first argument, got {:?}",
            other
        )),
    }
}

/// Array::partition(arr, f) – splits array into [matches, nonMatches].
pub(crate) fn array_partition(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::partition expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            let mut matches = Vec::new();
            let mut non_matches = Vec::new();

            for elem in elems {
                let predicate = apply_function_with_this(
                    func.clone(),
                    vec![elem.clone()],
                    None,
                    Rc::clone(&env),
                );

                match predicate {
                    Object::Boolean(true) => matches.push(elem),
                    Object::Boolean(false) => non_matches.push(elem),
                    Object::Error(_) => return predicate,
                    other => {
                        return Object::error(format!(
                            "Array::partition predicate must return boolean, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Array(vec![Object::Array(matches), Object::Array(non_matches)])
        }
        other => Object::error(format!(
            "Array::partition expects an array as first argument, got {:?}",
            other
        )),
    }
}

/// Array::fill(value, n) – creates an array of n copies of value.
pub(crate) fn array_fill(args: Vec<Object>, _env: EnvRef) -> Object {
    let (value, n) = match expect_two_args(args, "Array::fill") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let n_val = match n {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Array::fill expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    if n_val < 0 {
        return Object::error("Array::fill count must be non-negative");
    }

    let result: Vec<Object> = std::iter::repeat(value).take(n_val as usize).collect();
    Object::Array(result)
}

/// Array::isEmpty(arr) – returns true if array has no elements.
pub(crate) fn array_is_empty(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::isEmpty") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => Object::Boolean(elems.is_empty()),
        other => Object::error(format!(
            "Array::isEmpty expects an array, got {:?}",
            other
        )),
    }
}

/// Array::forEach(arr, f) – calls f(element) for each element, returns null.
pub(crate) fn array_for_each(mut args: Vec<Object>, env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Array::forEach expects exactly 2 arguments (array, fn)");
    }

    let func = args.pop().unwrap();
    let arr = args.pop().unwrap();

    match arr {
        Object::Array(elems) => {
            for elem in elems {
                let result = apply_function_with_this(
                    func.clone(),
                    vec![elem],
                    None,
                    Rc::clone(&env),
                );

                if result.is_error() {
                    return result;
                }
            }

            Object::Null
        }
        other => Object::error(format!(
            "Array::forEach expects an array as first argument, got {:?}",
            other
        )),
    }
}

/// Array::len(arr) – returns the length of the array.
pub(crate) fn array_len(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Array::len") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(elems) => Object::Integer(elems.len() as i64),
        other => Object::error(format!(
            "Array::len expects an array, got {:?}",
            other
        )),
    }
}
