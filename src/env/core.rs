use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

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
    Environment::new()
}

/// Create a new environment enclosed within an existing outer environment.
#[inline]
pub fn new_enclosed_env(outer: EnvRef) -> EnvRef {
    Environment::new_enclosed(outer)
}


