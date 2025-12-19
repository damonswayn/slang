use std::collections::HashMap;
use std::time::Duration;

use crate::env::EnvRef;
use crate::object::Object;

/// Converts a slang Object (HashMap) to HTTP headers
fn extract_headers(obj: &Object) -> Result<Vec<(String, String)>, String> {
    match obj {
        Object::Object(map) => {
            let mut headers = Vec::new();
            for (key, value) in map {
                match value {
                    Object::String(s) => headers.push((key.clone(), s.clone())),
                    other => {
                        return Err(format!(
                            "Header value must be a string, got {:?} for key '{}'",
                            other, key
                        ))
                    }
                }
            }
            Ok(headers)
        }
        _ => Err("Headers must be an object".to_string()),
    }
}

/// Converts a ureq Response to a slang Object
fn response_to_object(response: ureq::Response) -> Object {
    let status = response.status();
    let status_text = response.status_text().to_string();

    // Collect headers
    let mut headers_map = HashMap::new();
    for name in response.headers_names() {
        if let Some(value) = response.header(&name) {
            headers_map.insert(name, Object::String(value.to_string()));
        }
    }

    // Read body as string
    let body = response.into_string().unwrap_or_default();

    let mut result = HashMap::new();
    result.insert("status".to_string(), Object::Integer(status as i64));
    result.insert("statusText".to_string(), Object::String(status_text));
    result.insert("headers".to_string(), Object::Object(headers_map));
    result.insert("body".to_string(), Object::String(body));

    Object::Object(result)
}

/// HTTP::get(url) -> Result<{ status, statusText, headers, body }>
/// HTTP::get(url, options) -> Result<{ status, statusText, headers, body }>
/// options: { headers: { ... }, timeout: ms }
pub(crate) fn http_get(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.is_empty() || args.len() > 2 {
        return Object::error("HTTP::get expects 1 or 2 arguments (url, [options])");
    }

    let options = if args.len() == 2 { args.pop() } else { None };
    let url = args.pop().unwrap();

    let url_str = match url {
        Object::String(s) => s,
        other => {
            return Object::error(format!("HTTP::get expects string URL, got {:?}", other))
        }
    };

    let mut request = ureq::get(&url_str);

    // Apply options if provided
    if let Some(opts) = options {
        if let Object::Object(opts_map) = opts {
            // Apply headers
            if let Some(headers_obj) = opts_map.get("headers") {
                match extract_headers(headers_obj) {
                    Ok(headers) => {
                        for (key, value) in headers {
                            request = request.set(&key, &value);
                        }
                    }
                    Err(e) => return Object::error(e),
                }
            }

            // Apply timeout
            if let Some(Object::Integer(ms)) = opts_map.get("timeout") {
                request = request.timeout(Duration::from_millis(*ms as u64));
            }
        }
    }

    match request.call() {
        Ok(response) => Object::ResultOk(Box::new(response_to_object(response))),
        Err(ureq::Error::Status(code, response)) => {
            // HTTP error status (4xx, 5xx) - still return the response
            let result = match response_to_object(response) {
                Object::Object(mut map) => {
                    map.insert("status".to_string(), Object::Integer(code as i64));
                    Object::Object(map)
                }
                other => other,
            };
            Object::ResultErr(Box::new(result))
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "HTTP request failed: {}",
            e
        )))),
    }
}

/// HTTP::post(url, body) -> Result<{ status, statusText, headers, body }>
/// HTTP::post(url, body, options) -> Result<{ status, statusText, headers, body }>
pub(crate) fn http_post(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::error("HTTP::post expects 2 or 3 arguments (url, body, [options])");
    }

    let options = if args.len() == 3 { args.pop() } else { None };
    let body = args.pop().unwrap();
    let url = args.pop().unwrap();

    let url_str = match url {
        Object::String(s) => s,
        other => {
            return Object::error(format!("HTTP::post expects string URL, got {:?}", other))
        }
    };

    let body_str = match &body {
        Object::String(s) => s.clone(),
        // For objects/arrays, serialize to JSON
        Object::Object(_) | Object::Array(_) => {
            match serde_json::to_string(&object_to_json(&body)) {
                Ok(s) => s,
                Err(e) => return Object::error(format!("Failed to serialize body: {}", e)),
            }
        }
        other => {
            return Object::error(format!(
                "HTTP::post body must be string or object, got {:?}",
                other
            ))
        }
    };

    let mut request = ureq::post(&url_str);

    // Set content-type for JSON bodies
    if matches!(body, Object::Object(_) | Object::Array(_)) {
        request = request.set("Content-Type", "application/json");
    }

    // Apply options if provided
    if let Some(opts) = options {
        if let Object::Object(opts_map) = opts {
            if let Some(headers_obj) = opts_map.get("headers") {
                match extract_headers(headers_obj) {
                    Ok(headers) => {
                        for (key, value) in headers {
                            request = request.set(&key, &value);
                        }
                    }
                    Err(e) => return Object::error(e),
                }
            }

            if let Some(Object::Integer(ms)) = opts_map.get("timeout") {
                request = request.timeout(Duration::from_millis(*ms as u64));
            }
        }
    }

    match request.send_string(&body_str) {
        Ok(response) => Object::ResultOk(Box::new(response_to_object(response))),
        Err(ureq::Error::Status(code, response)) => {
            let result = match response_to_object(response) {
                Object::Object(mut map) => {
                    map.insert("status".to_string(), Object::Integer(code as i64));
                    Object::Object(map)
                }
                other => other,
            };
            Object::ResultErr(Box::new(result))
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "HTTP request failed: {}",
            e
        )))),
    }
}

/// HTTP::put(url, body) -> Result<{ status, statusText, headers, body }>
/// HTTP::put(url, body, options) -> Result<{ status, statusText, headers, body }>
pub(crate) fn http_put(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::error("HTTP::put expects 2 or 3 arguments (url, body, [options])");
    }

    let options = if args.len() == 3 { args.pop() } else { None };
    let body = args.pop().unwrap();
    let url = args.pop().unwrap();

    let url_str = match url {
        Object::String(s) => s,
        other => {
            return Object::error(format!("HTTP::put expects string URL, got {:?}", other))
        }
    };

    let body_str = match &body {
        Object::String(s) => s.clone(),
        Object::Object(_) | Object::Array(_) => {
            match serde_json::to_string(&object_to_json(&body)) {
                Ok(s) => s,
                Err(e) => return Object::error(format!("Failed to serialize body: {}", e)),
            }
        }
        other => {
            return Object::error(format!(
                "HTTP::put body must be string or object, got {:?}",
                other
            ))
        }
    };

    let mut request = ureq::put(&url_str);

    if matches!(body, Object::Object(_) | Object::Array(_)) {
        request = request.set("Content-Type", "application/json");
    }

    if let Some(opts) = options {
        if let Object::Object(opts_map) = opts {
            if let Some(headers_obj) = opts_map.get("headers") {
                match extract_headers(headers_obj) {
                    Ok(headers) => {
                        for (key, value) in headers {
                            request = request.set(&key, &value);
                        }
                    }
                    Err(e) => return Object::error(e),
                }
            }

            if let Some(Object::Integer(ms)) = opts_map.get("timeout") {
                request = request.timeout(Duration::from_millis(*ms as u64));
            }
        }
    }

    match request.send_string(&body_str) {
        Ok(response) => Object::ResultOk(Box::new(response_to_object(response))),
        Err(ureq::Error::Status(code, response)) => {
            let result = match response_to_object(response) {
                Object::Object(mut map) => {
                    map.insert("status".to_string(), Object::Integer(code as i64));
                    Object::Object(map)
                }
                other => other,
            };
            Object::ResultErr(Box::new(result))
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "HTTP request failed: {}",
            e
        )))),
    }
}

/// HTTP::delete(url) -> Result<{ status, statusText, headers, body }>
/// HTTP::delete(url, options) -> Result<{ status, statusText, headers, body }>
pub(crate) fn http_delete(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.is_empty() || args.len() > 2 {
        return Object::error("HTTP::delete expects 1 or 2 arguments (url, [options])");
    }

    let options = if args.len() == 2 { args.pop() } else { None };
    let url = args.pop().unwrap();

    let url_str = match url {
        Object::String(s) => s,
        other => {
            return Object::error(format!("HTTP::delete expects string URL, got {:?}", other))
        }
    };

    let mut request = ureq::delete(&url_str);

    if let Some(opts) = options {
        if let Object::Object(opts_map) = opts {
            if let Some(headers_obj) = opts_map.get("headers") {
                match extract_headers(headers_obj) {
                    Ok(headers) => {
                        for (key, value) in headers {
                            request = request.set(&key, &value);
                        }
                    }
                    Err(e) => return Object::error(e),
                }
            }

            if let Some(Object::Integer(ms)) = opts_map.get("timeout") {
                request = request.timeout(Duration::from_millis(*ms as u64));
            }
        }
    }

    match request.call() {
        Ok(response) => Object::ResultOk(Box::new(response_to_object(response))),
        Err(ureq::Error::Status(code, response)) => {
            let result = match response_to_object(response) {
                Object::Object(mut map) => {
                    map.insert("status".to_string(), Object::Integer(code as i64));
                    Object::Object(map)
                }
                other => other,
            };
            Object::ResultErr(Box::new(result))
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "HTTP request failed: {}",
            e
        )))),
    }
}

/// HTTP::patch(url, body) -> Result<{ status, statusText, headers, body }>
/// HTTP::patch(url, body, options) -> Result<{ status, statusText, headers, body }>
pub(crate) fn http_patch(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::error("HTTP::patch expects 2 or 3 arguments (url, body, [options])");
    }

    let options = if args.len() == 3 { args.pop() } else { None };
    let body = args.pop().unwrap();
    let url = args.pop().unwrap();

    let url_str = match url {
        Object::String(s) => s,
        other => {
            return Object::error(format!("HTTP::patch expects string URL, got {:?}", other))
        }
    };

    let body_str = match &body {
        Object::String(s) => s.clone(),
        Object::Object(_) | Object::Array(_) => {
            match serde_json::to_string(&object_to_json(&body)) {
                Ok(s) => s,
                Err(e) => return Object::error(format!("Failed to serialize body: {}", e)),
            }
        }
        other => {
            return Object::error(format!(
                "HTTP::patch body must be string or object, got {:?}",
                other
            ))
        }
    };

    let mut request = ureq::patch(&url_str);

    if matches!(body, Object::Object(_) | Object::Array(_)) {
        request = request.set("Content-Type", "application/json");
    }

    if let Some(opts) = options {
        if let Object::Object(opts_map) = opts {
            if let Some(headers_obj) = opts_map.get("headers") {
                match extract_headers(headers_obj) {
                    Ok(headers) => {
                        for (key, value) in headers {
                            request = request.set(&key, &value);
                        }
                    }
                    Err(e) => return Object::error(e),
                }
            }

            if let Some(Object::Integer(ms)) = opts_map.get("timeout") {
                request = request.timeout(Duration::from_millis(*ms as u64));
            }
        }
    }

    match request.send_string(&body_str) {
        Ok(response) => Object::ResultOk(Box::new(response_to_object(response))),
        Err(ureq::Error::Status(code, response)) => {
            let result = match response_to_object(response) {
                Object::Object(mut map) => {
                    map.insert("status".to_string(), Object::Integer(code as i64));
                    Object::Object(map)
                }
                other => other,
            };
            Object::ResultErr(Box::new(result))
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "HTTP request failed: {}",
            e
        )))),
    }
}

/// HTTP::head(url) -> Result<{ status, statusText, headers }>
pub(crate) fn http_head(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.is_empty() || args.len() > 2 {
        return Object::error("HTTP::head expects 1 or 2 arguments (url, [options])");
    }

    let options = if args.len() == 2 { args.pop() } else { None };
    let url = args.pop().unwrap();

    let url_str = match url {
        Object::String(s) => s,
        other => {
            return Object::error(format!("HTTP::head expects string URL, got {:?}", other))
        }
    };

    let mut request = ureq::head(&url_str);

    if let Some(opts) = options {
        if let Object::Object(opts_map) = opts {
            if let Some(headers_obj) = opts_map.get("headers") {
                match extract_headers(headers_obj) {
                    Ok(headers) => {
                        for (key, value) in headers {
                            request = request.set(&key, &value);
                        }
                    }
                    Err(e) => return Object::error(e),
                }
            }

            if let Some(Object::Integer(ms)) = opts_map.get("timeout") {
                request = request.timeout(Duration::from_millis(*ms as u64));
            }
        }
    }

    match request.call() {
        Ok(response) => {
            let status = response.status();
            let status_text = response.status_text().to_string();

            let mut headers_map = HashMap::new();
            for name in response.headers_names() {
                if let Some(value) = response.header(&name) {
                    headers_map.insert(name, Object::String(value.to_string()));
                }
            }

            let mut result = HashMap::new();
            result.insert("status".to_string(), Object::Integer(status as i64));
            result.insert("statusText".to_string(), Object::String(status_text));
            result.insert("headers".to_string(), Object::Object(headers_map));

            Object::ResultOk(Box::new(Object::Object(result)))
        }
        Err(ureq::Error::Status(code, response)) => {
            let status_text = response.status_text().to_string();

            let mut headers_map = HashMap::new();
            for name in response.headers_names() {
                if let Some(value) = response.header(&name) {
                    headers_map.insert(name, Object::String(value.to_string()));
                }
            }

            let mut result = HashMap::new();
            result.insert("status".to_string(), Object::Integer(code as i64));
            result.insert("statusText".to_string(), Object::String(status_text));
            result.insert("headers".to_string(), Object::Object(headers_map));

            Object::ResultErr(Box::new(Object::Object(result)))
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "HTTP request failed: {}",
            e
        )))),
    }
}

/// Convert slang Object to serde_json::Value for serialization
fn object_to_json(obj: &Object) -> serde_json::Value {
    match obj {
        Object::Null => serde_json::Value::Null,
        Object::Boolean(b) => serde_json::Value::Bool(*b),
        Object::Integer(i) => serde_json::Value::Number((*i).into()),
        Object::Float(f) => {
            serde_json::Number::from_f64(*f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        }
        Object::String(s) => serde_json::Value::String(s.clone()),
        Object::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(object_to_json).collect())
        }
        Object::Object(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), object_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}

