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
        // Pre-bind namespaces Option, Result, Regex, File, Array and Test.
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

        // Array = { map, filter, reduce }
        let mut array_methods = HashMap::new();
        array_methods.insert("map".to_string(), Object::Builtin(array_map));
        array_methods.insert("filter".to_string(), Object::Builtin(array_filter));
        array_methods.insert("reduce".to_string(), Object::Builtin(array_reduce));
        inner.set("Array".to_string(), Object::Object(array_methods));

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


