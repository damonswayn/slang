use std::fmt::{self, Display, Formatter};
use crate::ast::{BlockStatement, Identifier};
use crate::env::EnvRef;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Function {
        params: Vec<Identifier>,
        body: BlockStatement,
        env: EnvRef,
    },
    Builtin(BuiltinFunction),
    ReturnValue(Box<Object>),
    Null,
}

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object::*;

        match (self, other) {
            (Integer(a), Integer(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Boolean(a), Boolean(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Array(a), Array(b)) => a == b,
            // Functions and builtins are not compared for equality in this interpreter,
            // so we conservatively treat them as unequal (except by identity via reference,
            // which the current code never relies on).
            (Function { .. }, Function { .. }) => false,
            (Builtin(_), Builtin(_)) => false,
            (ReturnValue(a), ReturnValue(b)) => a == b,
            (Null, Null) => true,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(x) => write!(f, "{}", x),
            Object::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Array(elements) => {
                let inner = elements
                    .iter()
                    .map(|o| format!("{}", o))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", inner)
            },
            Object::Function { .. } => write!(f, "<user fn>"),
            Object::Builtin(_) => write!(f, "<native fn>"),
            Object::ReturnValue(obj) => write!(f, "{}", obj.to_string()),
            Object::Null => write!(f, "null"),
        }
    }
}