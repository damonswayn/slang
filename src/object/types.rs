use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(x) => write!(f, "{}", x),
            Object::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Object::Null => write!(f, "null"),
        }
    }
}