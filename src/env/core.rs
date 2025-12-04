use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;
use crate::builtins::native::monad_builtins::{
    option_some,
    option_none,
    option_is_some,
    option_is_none,
    option_unwrap_or,
    option_map,
    option_and_then,
    option_bind,
    option_fmap,
    result_ok,
    result_err,
    result_is_ok,
    result_is_err,
    result_unwrap_or,
    result_map,
    result_and_then,
    result_bind,
    result_fmap,
};
use crate::builtins::native::regex_builtins::{
    builtin_regex_is_match,
    builtin_regex_find,
    builtin_regex_replace,
    builtin_regex_match,
};
use crate::builtins::native::file_builtins::{
    file_open_result,
    file_read_result,
    file_write_result,
    file_seek_result,
    file_close_result,
};
use crate::builtins::native::test_builtins::{
    test_assert,
    test_assert_eq,
    test_assert_not_eq,
};
use crate::builtins::native::array_builtins::{
    array_map,
    array_filter,
    array_reduce,
    array_find,
    array_some,
    array_every,
    array_flat_map,
};
use crate::builtins::native::math_builtins::{
    math_abs,
    math_floor,
    math_ceil,
    math_round,
    math_min,
    math_max,
    math_pow,
    math_sin,
    math_cos,
    math_tan,
    math_sqrt,
};
use crate::builtins::native::string_builtins::{
    string_trim,
    string_to_upper,
    string_to_lower,
    string_split,
    string_join,
};
use crate::builtins::native::json_builtins::{
    json_parse,
    json_stringify,
};

/// Reference-counted, interior-mutable environment handle
pub type EnvRef = Rc<RefCell<Environment>>;

/// Simple lexical environment for variables
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed(outer: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(val) = self.store.get(name) {
            Some(val.clone())
        } else if let Some(ref outer) = self.outer {
            outer.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

/// Create a new, top-level environment.
#[inline]
pub fn new_env() -> EnvRef {
    let env = Environment::new();

    {
        // Pre-bind namespaces Option, Result, Regex, File, Array, Math, String, Json and Test.
        let mut inner = env.borrow_mut();

        // Option = { Some, None, isSome, isNone, unwrapOr, map, andThen, bind, fmap }
        let mut option_methods = HashMap::new();
        option_methods.insert("Some".to_string(), Object::Builtin(option_some));
        option_methods.insert("None".to_string(), Object::Builtin(option_none));
        option_methods.insert("isSome".to_string(), Object::Builtin(option_is_some));
        option_methods.insert("isNone".to_string(), Object::Builtin(option_is_none));
        option_methods.insert("unwrapOr".to_string(), Object::Builtin(option_unwrap_or));
        option_methods.insert("map".to_string(), Object::Builtin(option_map));
        option_methods.insert("andThen".to_string(), Object::Builtin(option_and_then));
        option_methods.insert("bind".to_string(), Object::Builtin(option_bind));
        option_methods.insert("fmap".to_string(), Object::Builtin(option_fmap));
        inner.set("Option".to_string(), Object::Object(option_methods));

        // Result = { Ok, Err, isOk, isErr, unwrapOr, map, andThen, bind, fmap }
        let mut result_methods = HashMap::new();
        result_methods.insert("Ok".to_string(), Object::Builtin(result_ok));
        result_methods.insert("Err".to_string(), Object::Builtin(result_err));
        result_methods.insert("isOk".to_string(), Object::Builtin(result_is_ok));
        result_methods.insert("isErr".to_string(), Object::Builtin(result_is_err));
        result_methods.insert("unwrapOr".to_string(), Object::Builtin(result_unwrap_or));
        result_methods.insert("map".to_string(), Object::Builtin(result_map));
        result_methods.insert("andThen".to_string(), Object::Builtin(result_and_then));
        result_methods.insert("bind".to_string(), Object::Builtin(result_bind));
        result_methods.insert("fmap".to_string(), Object::Builtin(result_fmap));
        inner.set("Result".to_string(), Object::Object(result_methods));

        // Regex = { isMatch, find, replace, match }
        let mut regex_methods = HashMap::new();
        regex_methods.insert("isMatch".to_string(), Object::Builtin(builtin_regex_is_match));
        regex_methods.insert("find".to_string(), Object::Builtin(builtin_regex_find));
        regex_methods.insert("replace".to_string(), Object::Builtin(builtin_regex_replace));
        regex_methods.insert("match".to_string(), Object::Builtin(builtin_regex_match));
        inner.set("Regex".to_string(), Object::Object(regex_methods));

        // File = { open, read, write, seek, close } – Result-based wrappers
        let mut file_methods = HashMap::new();
        file_methods.insert("open".to_string(), Object::Builtin(file_open_result));
        file_methods.insert("read".to_string(), Object::Builtin(file_read_result));
        file_methods.insert("write".to_string(), Object::Builtin(file_write_result));
        file_methods.insert("seek".to_string(), Object::Builtin(file_seek_result));
        file_methods.insert("close".to_string(), Object::Builtin(file_close_result));
        inner.set("File".to_string(), Object::Object(file_methods));

        // Array = { map, filter, reduce, find, some, every, flatMap }
        let mut array_methods = HashMap::new();
        array_methods.insert("map".to_string(), Object::Builtin(array_map));
        array_methods.insert("filter".to_string(), Object::Builtin(array_filter));
        array_methods.insert("reduce".to_string(), Object::Builtin(array_reduce));
        array_methods.insert("find".to_string(), Object::Builtin(array_find));
        array_methods.insert("some".to_string(), Object::Builtin(array_some));
        array_methods.insert("every".to_string(), Object::Builtin(array_every));
        array_methods.insert("flatMap".to_string(), Object::Builtin(array_flat_map));
        inner.set("Array".to_string(), Object::Object(array_methods));

        // Math = { abs, floor, ceil, round, min, max, pow, sin, cos, tan, sqrt }
        let mut math_methods = HashMap::new();
        math_methods.insert("abs".to_string(), Object::Builtin(math_abs));
        math_methods.insert("floor".to_string(), Object::Builtin(math_floor));
        math_methods.insert("ceil".to_string(), Object::Builtin(math_ceil));
        math_methods.insert("round".to_string(), Object::Builtin(math_round));
        math_methods.insert("min".to_string(), Object::Builtin(math_min));
        math_methods.insert("max".to_string(), Object::Builtin(math_max));
        math_methods.insert("pow".to_string(), Object::Builtin(math_pow));
        math_methods.insert("sin".to_string(), Object::Builtin(math_sin));
        math_methods.insert("cos".to_string(), Object::Builtin(math_cos));
        math_methods.insert("tan".to_string(), Object::Builtin(math_tan));
        math_methods.insert("sqrt".to_string(), Object::Builtin(math_sqrt));
        inner.set("Math".to_string(), Object::Object(math_methods));

        // String = { trim, toUpper, toLower, split, join }
        let mut string_methods = HashMap::new();
        string_methods.insert("trim".to_string(), Object::Builtin(string_trim));
        string_methods.insert("toUpper".to_string(), Object::Builtin(string_to_upper));
        string_methods.insert("toLower".to_string(), Object::Builtin(string_to_lower));
        string_methods.insert("split".to_string(), Object::Builtin(string_split));
        string_methods.insert("join".to_string(), Object::Builtin(string_join));
        inner.set("String".to_string(), Object::Object(string_methods));

        // Json = { parse, stringify }
        let mut json_methods = HashMap::new();
        json_methods.insert("parse".to_string(), Object::Builtin(json_parse));
        json_methods.insert("stringify".to_string(), Object::Builtin(json_stringify));
        inner.set("Json".to_string(), Object::Object(json_methods));

        // Test = { assert, assertEq, assertNotEq }
        let mut test_methods = HashMap::new();
        test_methods.insert("assert".to_string(), Object::Builtin(test_assert));
        test_methods.insert("assertEq".to_string(), Object::Builtin(test_assert_eq));
        test_methods.insert("assertNotEq".to_string(), Object::Builtin(test_assert_not_eq));
        inner.set("Test".to_string(), Object::Object(test_methods));
    }

    env
}

/// Create a new environment enclosed within an existing outer environment.
#[inline]
pub fn new_enclosed_env(outer: EnvRef) -> EnvRef {
    Environment::new_enclosed(outer)
}


