use crate::ast::{BlockStatement, Identifier};
use crate::env::EnvRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Object {
    // Primitive scalar types
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),

    // Compound data structures
    Array(Vec<Object>),
    Object(HashMap<String, Object>),

    // Functions (user-defined and native)
    Function {
        params: Vec<Identifier>,
        body: BlockStatement,
        env: EnvRef,
    },
    Builtin(BuiltinFunction),

    // Classes
    Class {
        name: String,
        methods: HashMap<String, Object>,
    },

    // Control-flow / special runtime values
    ReturnValue(Box<Object>),

    // IO
    File(FileRef),

    // Error handling
    Error(String),

    // Algebraic data types / monads
    /// An optional value: `Some(v)` or `None`.
    OptionSome(Box<Object>),
    OptionNone,

    /// A result of a computation: `Ok(v)` or `Err(e)`.
    ResultOk(Box<Object>),
    ResultErr(Box<Object>),

    // Null / unit
    Null,
}

/// Native builtin function type. Builtins receive the evaluated argument list
/// and the calling environment, so they can (optionally) call back into the
/// evaluator via higher-order helpers.
pub type BuiltinFunction = fn(Vec<Object>, EnvRef) -> Object;

pub type FileRef = Rc<RefCell<FileHandle>>;
#[derive(Debug)]
pub struct FileHandle {
    pub inner: Option<File>,
}

impl FileHandle {
    pub fn new(f: File) -> Self {
        Self { inner: Some(f) }
    }

    pub fn is_closed(&self) -> bool {
        self.inner.is_none()
    }
}

impl Object {
    pub fn error<S: Into<String>>(msg: S) -> Self {
        Object::Error(msg.into())
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use self::Object::*;

        match (self, other) {
            (Integer(a), Integer(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (Boolean(a), Boolean(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Array(a), Array(b)) => a == b,
            (Object(a), Object(b)) => a == b,
            // Functions and builtins are not compared for equality in this interpreter,
            // so we conservatively treat them as unequal (except by identity via reference,
            // which the current code never relies on).
            (Function { .. }, Function { .. }) => false,
            (Builtin(_), Builtin(_)) => false,
            (Class { .. }, Class { .. }) => false,
            (ReturnValue(a), ReturnValue(b)) => a == b,
            (File(_), File(_)) => false,
            (Error(a), Error(b)) => a == b,
            (OptionSome(a), OptionSome(b)) => a == b,
            (OptionNone, OptionNone) => true,
            (ResultOk(a), ResultOk(b)) => a == b,
            (ResultErr(a), ResultErr(b)) => a == b,
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
            }
            Object::Object(map) => {
                let mut parts = Vec::with_capacity(map.len());
                for (k, v) in map {
                    parts.push(format!("{}: {}", k, v));
                }
                write!(f, "{{{}}}", parts.join(", "))
            }
            Object::Function { .. } => write!(f, "<user fn>"),
            Object::Builtin(_) => write!(f, "<native fn>"),
            Object::Class { name, .. } => write!(f, "<class {}>", name),
            Object::ReturnValue(obj) => write!(f, "{}", obj.to_string()),
            Object::File(_) => write!(f, "<file>"),
            Object::Error(msg) => write!(f, "{}", msg),
            Object::OptionSome(inner) => write!(f, "Some({})", inner),
            Object::OptionNone => write!(f, "None"),
            Object::ResultOk(inner) => write!(f, "Ok({})", inner),
            Object::ResultErr(inner) => write!(f, "Err({})", inner),
            Object::Null => write!(f, "null"),
        }
    }
}
