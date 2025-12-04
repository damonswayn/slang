use crate::env::EnvRef;
use crate::object::Object;

/// Convert a serde_json::Value into a Slang Object.
fn from_json_value(v: &serde_json::Value) -> Object {
    use serde_json::Value;

    match v {
        Value::Null => Object::Null,
        Value::Bool(b) => Object::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Object::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Object::Float(f)
            } else {
                Object::Null
            }
        }
        Value::String(s) => Object::String(s.clone()),
        Value::Array(arr) => {
            let elements = arr.iter().map(from_json_value).collect();
            Object::Array(elements)
        }
        Value::Object(map) => {
            let mut out = std::collections::HashMap::new();
            for (k, v) in map {
                out.insert(k.clone(), from_json_value(v));
            }
            Object::Object(out)
        }
    }
}

/// Convert a Slang Object into a serde_json::Value.
fn to_json_value(obj: &Object) -> serde_json::Value {
    use serde_json::Value;

    match obj {
        Object::Null => Value::Null,
        Object::Boolean(b) => Value::Bool(*b),
        Object::Integer(i) => Value::Number(serde_json::Number::from(*i)),
        Object::Float(f) => {
            let n = serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0));
            Value::Number(n)
        }
        Object::String(s) => Value::String(s.clone()),
        Object::Array(elems) => {
            let arr = elems.iter().map(to_json_value).collect();
            Value::Array(arr)
        }
        Object::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                out.insert(k.clone(), to_json_value(v));
            }
            Value::Object(out)
        }
        // Fallback: use debug representation for unsupported values
        other => Value::String(format!("{:?}", other)),
    }
}

/// Json::parse(s) -> Result::Ok(value) or Result::Err(errorString)
pub(crate) fn json_parse(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Json::parse expects exactly 1 argument (string)");
    }

    let s = match &args[0] {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Json::parse expects string as first argument, got {:?}",
                other
            ))
        }
    };

    match serde_json::from_str::<serde_json::Value>(s) {
        Ok(v) => Object::ResultOk(Box::new(from_json_value(&v))),
        Err(e) => Object::ResultErr(Box::new(Object::String(e.to_string()))),
    }
}

/// Json::stringify(value) -> Result::Ok(string) or Result::Err(errorString)
pub(crate) fn json_stringify(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 1 {
        return Object::error("Json::stringify expects exactly 1 argument (value)");
    }

    let v = &args[0];

    let json_value = to_json_value(v);
    let s = serde_json::to_string(&json_value)
        .unwrap_or_else(|e| format!("<error: {}>", e));
    Object::ResultOk(Box::new(Object::String(s)))
}


