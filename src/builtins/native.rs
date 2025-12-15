use crate::env::EnvRef;
use crate::object::Object;
use crate::object::Object::Integer;
use crate::object::types::BuiltinFunction;

pub mod monad_builtins;
pub mod file_builtins;
pub mod regex_builtins;
pub mod test_builtins;
pub mod array_builtins;
pub mod math_builtins;
pub mod string_builtins;
pub mod json_builtins;
pub mod type_builtins;

// Re-export file builtins so other modules (like env) can
// attach them under namespaces without knowing the submodule path.
pub use file_builtins::{
    builtin_open,
    builtin_read,
    builtin_write,
    builtin_seek,
    builtin_close,
};

pub struct Builtin {
    pub name: &'static str,
    pub func: BuiltinFunction,
}

const BUILTINS: &[Builtin] = &[
    Builtin { name: "len",   func: builtin_len },
    Builtin { name: "first", func: builtin_first },
    Builtin { name: "last",  func: builtin_last },
    Builtin { name: "rest",  func: builtin_rest },
    Builtin { name: "push",  func: builtin_push },
    Builtin { name: "print", func: builtin_print },
    Builtin { name: "debug", func: builtin_debug },
    Builtin { name: "int", func: type_builtins::builtin_int },
    Builtin { name: "float", func: type_builtins::builtin_float },
    Builtin { name: "str", func: type_builtins::builtin_str },
    Builtin { name: "bool", func: type_builtins::builtin_bool },

    // Regex builtins
    Builtin { name: "regexIsMatch", func: regex_builtins::builtin_regex_is_match },
    Builtin { name: "regexFind", func: regex_builtins::builtin_regex_find },
    Builtin { name: "regexReplace", func: regex_builtins::builtin_regex_replace },
    Builtin { name: "regexMatch", func: regex_builtins::builtin_regex_match },

    // File builtins
    Builtin { name: "file_open", func: file_builtins::builtin_open },
    Builtin { name: "file_read", func: file_builtins::builtin_read },
    Builtin { name: "file_write", func: file_builtins::builtin_write },
    Builtin { name: "file_seek", func: file_builtins::builtin_seek },
    Builtin { name: "file_close", func: file_builtins::builtin_close },

    // Test helpers (available via the `Test` namespace)
    Builtin { name: "test_assert", func: test_builtins::test_assert },
    Builtin { name: "test_assert_eq", func: test_builtins::test_assert_eq },
    Builtin { name: "test_assert_not_eq", func: test_builtins::test_assert_not_eq },
];

pub fn get(name: &str) -> Option<BuiltinFunction> {
    for b in BUILTINS {
        if b.name == name {
            return Some(b.func);
        }
    }
    None
}

// Native func implementations

fn builtin_len(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("len expects exactly 1 argument");
    }

    match &args[0] {
        Object::String(s) => Integer(s.len() as i64),
        Object::Array(a) => Integer(a.len() as i64),
        other => Object::error(format!("len not supported for value: {:?}", other)),
    }
}

fn builtin_first(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("first expects exactly 1 argument");
    }

    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() {
                Object::Null
            } else {
                elems[0].clone()
            }
        }
        other => Object::error(format!("first expects array, got {:?}", other)),
    }
}

fn builtin_last(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("last expects exactly 1 argument");
    }

    match &args[0] {
        Object::Array(elems) => {
            if let Some(last) = elems.last() {
                last.clone()
            } else {
                Object::Null
            }
        }
        other => Object::error(format!("last expects array, got {:?}", other)),
    }
}

fn builtin_rest(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("rest expects exactly 1 argument");
    }

    match &args[0] {
        Object::Array(elems) => {
            if elems.len() <= 1 {
                Object::Array(vec![])
            } else {
                Object::Array(elems[1..].to_vec())
            }
        }
        other => Object::error(format!("rest expects array, got {:?}", other)),
    }
}

fn builtin_push(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("push expects exactly 2 arguments");
    }

    let value = args.pop().unwrap();
    let array = args.pop().unwrap();

    match array {
        Object::Array(mut elems) => {
            elems.push(value);
            Object::Array(elems)
        }
        other => Object::error(format!("push expects array as first argument, got {:?}", other)),
    }
}

fn builtin_print(args: Vec<Object>, _env: EnvRef) -> Object {
    // Just print all args separated by space and newline
    let text = args
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", text);
    Object::Null
}

fn builtin_debug(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("debug expects exactly 1 argument");
    }

    match &args[0] {
        Object::Boolean(true) => {
            crate::debug::enable_debug_mode();
            Object::Boolean(true)
        },
        Object::Boolean(false) => {
            crate::debug::disable_debug_mode();
            Object::Boolean(false)
        },
        other => Object::error(format!("debug expects boolean, got {:?}", other)),
    }
}