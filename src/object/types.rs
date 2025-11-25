use std::fmt::{self, Display, Formatter};
use crate::ast::{BlockStatement, Identifier};
use crate::evaluator::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Function {
        params: Vec<Identifier>,
        body: BlockStatement,
        env: Environment,
    },
    Builtin(BuiltinFunction),
    ReturnValue(Box<Object>),
    Null,
}

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

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