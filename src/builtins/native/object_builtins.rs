use std::collections::HashMap;

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

/// Object::keys(obj) -> Array<String>
/// Returns an array of all keys in the object.
pub(crate) fn object_keys(args: Vec<Object>, _env: EnvRef) -> Object {
    let obj = match expect_one_arg(args, "Object::keys") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match obj {
        Object::Object(map) => {
            let keys: Vec<Object> = map
                .keys()
                .map(|k| Object::String(k.clone()))
                .collect();
            Object::Array(keys)
        }
        other => Object::error(format!(
            "Object::keys expects an object, got {:?}",
            other
        )),
    }
}

/// Object::values(obj) -> Array<any>
/// Returns an array of all values in the object.
pub(crate) fn object_values(args: Vec<Object>, _env: EnvRef) -> Object {
    let obj = match expect_one_arg(args, "Object::values") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match obj {
        Object::Object(map) => {
            let values: Vec<Object> = map.values().cloned().collect();
            Object::Array(values)
        }
        other => Object::error(format!(
            "Object::values expects an object, got {:?}",
            other
        )),
    }
}

/// Object::entries(obj) -> Array<[key, value]>
/// Returns an array of [key, value] pairs.
pub(crate) fn object_entries(args: Vec<Object>, _env: EnvRef) -> Object {
    let obj = match expect_one_arg(args, "Object::entries") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match obj {
        Object::Object(map) => {
            let entries: Vec<Object> = map
                .into_iter()
                .map(|(k, v)| Object::Array(vec![Object::String(k), v]))
                .collect();
            Object::Array(entries)
        }
        other => Object::error(format!(
            "Object::entries expects an object, got {:?}",
            other
        )),
    }
}

/// Object::fromEntries(arr) -> Object
/// Creates an object from an array of [key, value] pairs.
pub(crate) fn object_from_entries(args: Vec<Object>, _env: EnvRef) -> Object {
    let arr = match expect_one_arg(args, "Object::fromEntries") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match arr {
        Object::Array(entries) => {
            let mut map = HashMap::new();

            for entry in entries {
                match entry {
                    Object::Array(pair) if pair.len() == 2 => {
                        let key = match &pair[0] {
                            Object::String(s) => s.clone(),
                            other => {
                                return Object::error(format!(
                                    "Object::fromEntries expects string keys, got {:?}",
                                    other
                                ))
                            }
                        };
                        let value = pair[1].clone();
                        map.insert(key, value);
                    }
                    other => {
                        return Object::error(format!(
                            "Object::fromEntries expects [key, value] pairs, got {:?}",
                            other
                        ))
                    }
                }
            }

            Object::Object(map)
        }
        other => Object::error(format!(
            "Object::fromEntries expects an array, got {:?}",
            other
        )),
    }
}

/// Object::has(obj, key) -> bool
/// Returns true if the object has the given key.
pub(crate) fn object_has(args: Vec<Object>, _env: EnvRef) -> Object {
    let (obj, key) = match expect_two_args(args, "Object::has") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let key_str = match key {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Object::has expects string key, got {:?}",
                other
            ))
        }
    };

    match obj {
        Object::Object(map) => Object::Boolean(map.contains_key(&key_str)),
        other => Object::error(format!(
            "Object::has expects an object as first argument, got {:?}",
            other
        )),
    }
}

/// Object::get(obj, key) -> Option<value>
/// Returns Option::Some(value) if the key exists, Option::None otherwise.
pub(crate) fn object_get(args: Vec<Object>, _env: EnvRef) -> Object {
    let (obj, key) = match expect_two_args(args, "Object::get") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let key_str = match key {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Object::get expects string key, got {:?}",
                other
            ))
        }
    };

    match obj {
        Object::Object(map) => match map.get(&key_str) {
            Some(value) => Object::OptionSome(Box::new(value.clone())),
            None => Object::OptionNone,
        },
        other => Object::error(format!(
            "Object::get expects an object as first argument, got {:?}",
            other
        )),
    }
}

/// Object::set(obj, key, value) -> Object
/// Returns a new object with the key set to value (immutable).
pub(crate) fn object_set(args: Vec<Object>, _env: EnvRef) -> Object {
    let (obj, key, value) = match expect_three_args(args, "Object::set") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let key_str = match key {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Object::set expects string key, got {:?}",
                other
            ))
        }
    };

    match obj {
        Object::Object(mut map) => {
            map.insert(key_str, value);
            Object::Object(map)
        }
        other => Object::error(format!(
            "Object::set expects an object as first argument, got {:?}",
            other
        )),
    }
}

/// Object::delete(obj, key) -> Object
/// Returns a new object with the key removed (immutable).
pub(crate) fn object_delete(args: Vec<Object>, _env: EnvRef) -> Object {
    let (obj, key) = match expect_two_args(args, "Object::delete") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let key_str = match key {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Object::delete expects string key, got {:?}",
                other
            ))
        }
    };

    match obj {
        Object::Object(mut map) => {
            map.remove(&key_str);
            Object::Object(map)
        }
        other => Object::error(format!(
            "Object::delete expects an object as first argument, got {:?}",
            other
        )),
    }
}

/// Object::merge(obj1, obj2) -> Object
/// Returns a new object with all keys from both objects.
/// Keys from obj2 override keys from obj1.
pub(crate) fn object_merge(args: Vec<Object>, _env: EnvRef) -> Object {
    let (obj1, obj2) = match expect_two_args(args, "Object::merge") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let map1 = match obj1 {
        Object::Object(m) => m,
        other => {
            return Object::error(format!(
                "Object::merge expects an object as first argument, got {:?}",
                other
            ))
        }
    };

    let map2 = match obj2 {
        Object::Object(m) => m,
        other => {
            return Object::error(format!(
                "Object::merge expects an object as second argument, got {:?}",
                other
            ))
        }
    };

    let mut result = map1;
    for (k, v) in map2 {
        result.insert(k, v);
    }

    Object::Object(result)
}

/// Object::isEmpty(obj) -> bool
/// Returns true if the object has no keys.
pub(crate) fn object_is_empty(args: Vec<Object>, _env: EnvRef) -> Object {
    let obj = match expect_one_arg(args, "Object::isEmpty") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match obj {
        Object::Object(map) => Object::Boolean(map.is_empty()),
        other => Object::error(format!(
            "Object::isEmpty expects an object, got {:?}",
            other
        )),
    }
}

/// Object::len(obj) -> int
/// Returns the number of keys in the object.
pub(crate) fn object_len(args: Vec<Object>, _env: EnvRef) -> Object {
    let obj = match expect_one_arg(args, "Object::len") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match obj {
        Object::Object(map) => Object::Integer(map.len() as i64),
        other => Object::error(format!(
            "Object::len expects an object, got {:?}",
            other
        )),
    }
}

