use std::fmt::{self, Display, Formatter};
use crate::ast::{BlockStatement, Identifier};
use crate::evaluator::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Function {
        params: Vec<Identifier>,
        body: BlockStatement,
        env: Environment,
    },
    ReturnValue(Box<Object>),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(x) => write!(f, "{}", x),
            Object::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Object::Function { .. } => write!(f, "<native fn>"),
            Object::ReturnValue(obj) => write!(f, "{}", obj.to_string()),
            Object::Null => write!(f, "null"),
        }
    }
}