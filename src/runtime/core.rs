//! Runtime-facing API: values, environments, evaluator and builtins.
//!
//! This is a thin logical namespace over the existing modules; it does not
//! introduce new behavior, only groups related runtime pieces together.

pub use crate::env::{Environment, EnvRef};
pub use crate::object::Object;
pub use crate::evaluator::eval;
pub use crate::builtins::get as get_builtin;


