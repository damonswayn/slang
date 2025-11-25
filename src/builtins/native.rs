use crate::object::Object;
use crate::object::Object::Integer;
use crate::object::types::BuiltinFunction;

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

fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Null;
    }

    match &args[0] {
        Object::String(s) => Integer(s.len() as i64),
        Object::Array(a) => Integer(a.len() as i64),
        _ => Object::Null,
    }
}

fn builtin_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Null;
    }

    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() {
                Object::Null
            } else {
                elems[0].clone()
            }
        }
        _ => Object::Null,
    }
}

fn builtin_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Null;
    }

    match &args[0] {
        Object::Array(elems) => {
            if let Some(last) = elems.last() {
                last.clone()
            } else {
                Object::Null
            }
        }
        _ => Object::Null,
    }
}

fn builtin_rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Null;
    }

    match &args[0] {
        Object::Array(elems) => {
            if elems.len() <= 1 {
                Object::Array(vec![])
            } else {
                Object::Array(elems[1..].to_vec())
            }
        }
        _ => Object::Null,
    }
}

fn builtin_push(mut args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Null;
    }

    let value = args.pop().unwrap();
    let array = args.pop().unwrap();

    match array {
        Object::Array(mut elems) => {
            elems.push(value);
            Object::Array(elems)
        }
        _ => Object::Null,
    }
}

fn builtin_print(args: Vec<Object>) -> Object {
    // Just print all args separated by space and newline
    let text = args
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", text);
    Object::Null
}

fn builtin_debug(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Null
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
        _ => Object::Null
    }
}