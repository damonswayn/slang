use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;
use crate::builtins::native::{
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
    builtin_regex_is_match,
    builtin_regex_find,
    builtin_regex_replace,
    builtin_regex_match,
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
        // Pre-bind namespaces Option, Result and Regex.
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
    }

    env
}

/// Create a new environment enclosed within an existing outer environment.
#[inline]
pub fn new_enclosed_env(outer: EnvRef) -> EnvRef {
    Environment::new_enclosed(outer)
}


