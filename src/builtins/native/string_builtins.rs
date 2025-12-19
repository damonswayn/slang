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

/// String::contains(s, substr) -> bool
/// Returns true if s contains substr.
pub(crate) fn string_contains(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, substr) = match expect_two_args(args, "String::contains") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::contains expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let substr_val = match substr {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::contains expects string as second argument, got {:?}",
                other
            ))
        }
    };

    Object::Boolean(s_val.contains(&substr_val))
}

/// String::startsWith(s, prefix) -> bool
/// Returns true if s starts with prefix.
pub(crate) fn string_starts_with(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, prefix) = match expect_two_args(args, "String::startsWith") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::startsWith expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let prefix_val = match prefix {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::startsWith expects string as second argument, got {:?}",
                other
            ))
        }
    };

    Object::Boolean(s_val.starts_with(&prefix_val))
}

/// String::endsWith(s, suffix) -> bool
/// Returns true if s ends with suffix.
pub(crate) fn string_ends_with(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, suffix) = match expect_two_args(args, "String::endsWith") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::endsWith expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let suffix_val = match suffix {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::endsWith expects string as second argument, got {:?}",
                other
            ))
        }
    };

    Object::Boolean(s_val.ends_with(&suffix_val))
}

/// String::indexOf(s, substr) -> Option<int>
/// Returns Option::Some(index) of first occurrence, or Option::None if not found.
pub(crate) fn string_index_of(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, substr) = match expect_two_args(args, "String::indexOf") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::indexOf expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let substr_val = match substr {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::indexOf expects string as second argument, got {:?}",
                other
            ))
        }
    };

    match s_val.find(&substr_val) {
        Some(idx) => Object::OptionSome(Box::new(Object::Integer(idx as i64))),
        None => Object::OptionNone,
    }
}

/// String::slice(s, start, end) -> string
/// Returns substring from start (inclusive) to end (exclusive).
/// Negative indices count from end. Out of bounds are clamped.
pub(crate) fn string_slice(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, start, end) = match expect_three_args(args, "String::slice") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::slice expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let start_val = match start {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::slice expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    let end_val = match end {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::slice expects integer as third argument, got {:?}",
                other
            ))
        }
    };

    let len = s_val.chars().count() as i64;

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
        return Object::String(String::new());
    }

    let result: String = s_val
        .chars()
        .skip(start_idx)
        .take(end_idx - start_idx)
        .collect();

    Object::String(result)
}

/// String::replace(s, from, to) -> string
/// Replaces first occurrence of `from` with `to`.
pub(crate) fn string_replace(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, from, to) = match expect_three_args(args, "String::replace") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::replace expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let from_val = match from {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::replace expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let to_val = match to {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::replace expects string as third argument, got {:?}",
                other
            ))
        }
    };

    Object::String(s_val.replacen(&from_val, &to_val, 1))
}

/// String::repeat(s, n) -> string
/// Returns s repeated n times.
pub(crate) fn string_repeat(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, n) = match expect_two_args(args, "String::repeat") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::repeat expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let n_val = match n {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::repeat expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    if n_val < 0 {
        return Object::error("String::repeat count must be non-negative");
    }

    Object::String(s_val.repeat(n_val as usize))
}

/// String::reverse(s) -> string
/// Returns s with characters in reverse order.
pub(crate) fn string_reverse(args: Vec<Object>, _env: EnvRef) -> Object {
    let s = match expect_one_arg(args, "String::reverse") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match s {
        Object::String(v) => Object::String(v.chars().rev().collect()),
        other => Object::error(format!(
            "String::reverse expects a string, got {:?}",
            other
        )),
    }
}

/// String::padLeft(s, len, char) -> string
/// Pads s on the left to reach len using char.
pub(crate) fn string_pad_left(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, len, pad_char) = match expect_three_args(args, "String::padLeft") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::padLeft expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let len_val = match len {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::padLeft expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    let pad_char_val = match pad_char {
        Object::String(v) => {
            if v.chars().count() != 1 {
                return Object::error("String::padLeft expects single character as third argument");
            }
            v.chars().next().unwrap()
        }
        other => {
            return Object::error(format!(
                "String::padLeft expects string as third argument, got {:?}",
                other
            ))
        }
    };

    if len_val < 0 {
        return Object::error("String::padLeft length must be non-negative");
    }

    let current_len = s_val.chars().count();
    let target_len = len_val as usize;

    if current_len >= target_len {
        return Object::String(s_val);
    }

    let padding: String = std::iter::repeat(pad_char_val)
        .take(target_len - current_len)
        .collect();

    Object::String(format!("{}{}", padding, s_val))
}

/// String::padRight(s, len, char) -> string
/// Pads s on the right to reach len using char.
pub(crate) fn string_pad_right(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, len, pad_char) = match expect_three_args(args, "String::padRight") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::padRight expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let len_val = match len {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::padRight expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    let pad_char_val = match pad_char {
        Object::String(v) => {
            if v.chars().count() != 1 {
                return Object::error("String::padRight expects single character as third argument");
            }
            v.chars().next().unwrap()
        }
        other => {
            return Object::error(format!(
                "String::padRight expects string as third argument, got {:?}",
                other
            ))
        }
    };

    if len_val < 0 {
        return Object::error("String::padRight length must be non-negative");
    }

    let current_len = s_val.chars().count();
    let target_len = len_val as usize;

    if current_len >= target_len {
        return Object::String(s_val);
    }

    let padding: String = std::iter::repeat(pad_char_val)
        .take(target_len - current_len)
        .collect();

    Object::String(format!("{}{}", s_val, padding))
}

/// String::chars(s) -> array
/// Returns an array of single-character strings.
pub(crate) fn string_chars(args: Vec<Object>, _env: EnvRef) -> Object {
    let s = match expect_one_arg(args, "String::chars") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match s {
        Object::String(val) => {
            let chars: Vec<Object> = val
                .chars()
                .map(|c| Object::String(c.to_string()))
                .collect();
            Object::Array(chars)
        }
        other => Object::error(format!("String::chars expects a string, got {:?}", other)),
    }
}

/// String::charCodeAt(s, index) -> integer
/// Returns the Unicode code point at the given index.
pub(crate) fn string_char_code_at(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, idx) = match expect_two_args(args, "String::charCodeAt") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::charCodeAt expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let idx_val = match idx {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::charCodeAt expects integer as second argument, got {:?}",
                other
            ))
        }
    };

    if idx_val < 0 {
        return Object::error("String::charCodeAt index must be non-negative");
    }

    match s_val.chars().nth(idx_val as usize) {
        Some(c) => Object::Integer(c as i64),
        None => Object::error(format!(
            "String::charCodeAt index {} out of bounds for string of length {}",
            idx_val,
            s_val.chars().count()
        )),
    }
}

/// String::fromCharCode(code) -> string
/// Returns a string from a Unicode code point.
pub(crate) fn string_from_char_code(args: Vec<Object>, _env: EnvRef) -> Object {
    let code = match expect_one_arg(args, "String::fromCharCode") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let code_val = match code {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "String::fromCharCode expects integer, got {:?}",
                other
            ))
        }
    };

    if code_val < 0 {
        return Object::error("String::fromCharCode code point must be non-negative");
    }

    match char::from_u32(code_val as u32) {
        Some(c) => Object::String(c.to_string()),
        None => Object::error(format!(
            "String::fromCharCode invalid Unicode code point: {}",
            code_val
        )),
    }
}

/// String::fromCharCodes(codes) -> string
/// Returns a string from an array of Unicode code points.
pub(crate) fn string_from_char_codes(args: Vec<Object>, _env: EnvRef) -> Object {
    let codes = match expect_one_arg(args, "String::fromCharCodes") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let codes_arr = match codes {
        Object::Array(arr) => arr,
        other => {
            return Object::error(format!(
                "String::fromCharCodes expects array, got {:?}",
                other
            ))
        }
    };

    let mut result = String::new();
    for (i, code) in codes_arr.iter().enumerate() {
        match code {
            Object::Integer(val) => {
                if *val < 0 {
                    return Object::error(format!(
                        "String::fromCharCodes code point at index {} must be non-negative",
                        i
                    ));
                }
                match char::from_u32(*val as u32) {
                    Some(c) => result.push(c),
                    None => {
                        return Object::error(format!(
                            "String::fromCharCodes invalid Unicode code point at index {}: {}",
                            i, val
                        ))
                    }
                }
            }
            other => {
                return Object::error(format!(
                    "String::fromCharCodes expects array of integers, got {:?} at index {}",
                    other, i
                ))
            }
        }
    }

    Object::String(result)
}

/// String::lastIndexOf(s, substr) -> Option<integer>
/// Returns the last index of substr in s, or None if not found.
pub(crate) fn string_last_index_of(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, substr) = match expect_two_args(args, "String::lastIndexOf") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::lastIndexOf expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let substr_val = match substr {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::lastIndexOf expects string as second argument, got {:?}",
                other
            ))
        }
    };

    match s_val.rfind(&substr_val) {
        Some(idx) => Object::OptionSome(Box::new(Object::Integer(idx as i64))),
        None => Object::OptionNone,
    }
}

/// String::replaceAll(s, old, new) -> string
/// Replaces all occurrences of old with new in s.
pub(crate) fn string_replace_all(args: Vec<Object>, _env: EnvRef) -> Object {
    let (s, old, new) = match expect_three_args(args, "String::replaceAll") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let s_val = match s {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::replaceAll expects string as first argument, got {:?}",
                other
            ))
        }
    };

    let old_val = match old {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::replaceAll expects string as second argument, got {:?}",
                other
            ))
        }
    };

    let new_val = match new {
        Object::String(v) => v,
        other => {
            return Object::error(format!(
                "String::replaceAll expects string as third argument, got {:?}",
                other
            ))
        }
    };

    Object::String(s_val.replace(&old_val, &new_val))
}

/// String::charCodes(s) -> array
/// Returns an array of Unicode code points for each character in the string.
pub(crate) fn string_char_codes(args: Vec<Object>, _env: EnvRef) -> Object {
    let s = match expect_one_arg(args, "String::charCodes") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match s {
        Object::String(val) => {
            let codes: Vec<Object> = val
                .chars()
                .map(|c| Object::Integer(c as i64))
                .collect();
            Object::Array(codes)
        }
        other => Object::error(format!("String::charCodes expects a string, got {:?}", other)),
    }
}

/// String::isEmpty(s) -> boolean
/// Returns true if the string is empty.
pub(crate) fn string_is_empty(args: Vec<Object>, _env: EnvRef) -> Object {
    let s = match expect_one_arg(args, "String::isEmpty") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match s {
        Object::String(val) => Object::Boolean(val.is_empty()),
        other => Object::error(format!("String::isEmpty expects a string, got {:?}", other)),
    }
}

/// String::len(s) -> integer
/// Returns the number of characters (not bytes) in the string.
pub(crate) fn string_len(args: Vec<Object>, _env: EnvRef) -> Object {
    let s = match expect_one_arg(args, "String::len") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match s {
        Object::String(val) => Object::Integer(val.chars().count() as i64),
        other => Object::error(format!("String::len expects a string, got {:?}", other)),
    }
}
