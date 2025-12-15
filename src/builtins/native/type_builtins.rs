use crate::env::EnvRef;
use crate::object::Object;
use std::i64;

fn expect_one_arg(mut args: Vec<Object>, name: &str) -> Result<Object, Object> {
    if args.len() != 1 {
        return Err(Object::error(format!("{name} expects exactly 1 argument")));
    }
    Ok(args.pop().unwrap())
}

fn convert_to_int(value: Object) -> Result<Object, String> {
    match value {
        Object::Integer(i) => Ok(Object::Integer(i)),
        Object::Float(f) => {
            if !f.is_finite() {
                return Err("int(): cannot convert non-finite float".into());
            }

            if f < i64::MIN as f64 || f > i64::MAX as f64 {
                return Err("int(): float is out of range for i64".into());
            }

            Ok(Object::Integer(f.trunc() as i64))
        }
        Object::Boolean(b) => Ok(Object::Integer(if b { 1 } else { 0 })),
        Object::String(s) => match s.trim().parse::<i64>() {
            Ok(i) => Ok(Object::Integer(i)),
            Err(_) => Err(format!("int(): could not parse integer from \"{}\"", s)),
        },
        other => Err(format!("int(): cannot convert {:?} to int", other)),
    }
}

fn convert_to_float(value: Object) -> Result<Object, String> {
    match value {
        Object::Float(f) => {
            if !f.is_finite() {
                Err("float(): cannot convert non-finite float".into())
            } else {
                Ok(Object::Float(f))
            }
        }
        Object::Integer(i) => Ok(Object::Float(i as f64)),
        Object::Boolean(b) => Ok(Object::Float(if b { 1.0 } else { 0.0 })),
        Object::String(s) => match s.trim().parse::<f64>() {
            Ok(f) => Ok(Object::Float(f)),
            Err(_) => Err(format!("float(): could not parse float from \"{}\"", s)),
        },
        other => Err(format!("float(): cannot convert {:?} to float", other)),
    }
}

fn convert_to_string(value: Object) -> Result<Object, String> {
    match value {
        Object::String(s) => Ok(Object::String(s)),
        other => Ok(Object::String(other.to_string())),
    }
}

fn convert_to_bool(value: Object) -> Result<Object, String> {
    match value {
        Object::Boolean(b) => Ok(Object::Boolean(b)),
        Object::Integer(i) => Ok(Object::Boolean(i != 0)),
        Object::Float(f) => {
            if !f.is_finite() {
                Err("bool(): cannot convert non-finite float".into())
            } else {
                Ok(Object::Boolean(f != 0.0))
            }
        }
        Object::String(s) => {
            let lower = s.trim().to_ascii_lowercase();
            match lower.as_str() {
                "true" | "1" => Ok(Object::Boolean(true)),
                "false" | "0" => Ok(Object::Boolean(false)),
                _ => Err(format!("bool(): could not parse boolean from \"{}\"", s)),
            }
        }
        Object::Null => Ok(Object::Boolean(false)),
        other => Err(format!("bool(): cannot convert {:?} to bool", other)),
    }
}

fn wrap_result(res: Result<Object, String>) -> Object {
    match res {
        Ok(v) => Object::ResultOk(Box::new(v)),
        Err(msg) => Object::ResultErr(Box::new(Object::String(msg))),
    }
}

pub(crate) fn builtin_int(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "int") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match convert_to_int(value) {
        Ok(v) => v,
        Err(msg) => Object::error(msg),
    }
}

pub(crate) fn builtin_float(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "float") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match convert_to_float(value) {
        Ok(v) => v,
        Err(msg) => Object::error(msg),
    }
}

pub(crate) fn builtin_str(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "str") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match convert_to_string(value) {
        Ok(v) => v,
        Err(msg) => Object::error(msg),
    }
}

pub(crate) fn builtin_bool(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "bool") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match convert_to_bool(value) {
        Ok(v) => v,
        Err(msg) => Object::error(msg),
    }
}

pub(crate) fn type_int(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "Type::int") {
        Ok(v) => v,
        Err(e) => return e,
    };

    wrap_result(convert_to_int(value))
}

pub(crate) fn type_float(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "Type::float") {
        Ok(v) => v,
        Err(e) => return e,
    };

    wrap_result(convert_to_float(value))
}

pub(crate) fn type_str(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "Type::str") {
        Ok(v) => v,
        Err(e) => return e,
    };

    wrap_result(convert_to_string(value))
}

pub(crate) fn type_bool(args: Vec<Object>, _env: EnvRef) -> Object {
    let value = match expect_one_arg(args, "Type::bool") {
        Ok(v) => v,
        Err(e) => return e,
    };

    wrap_result(convert_to_bool(value))
}

