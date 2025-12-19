use crate::env::EnvRef;
use crate::object::Object;

fn expect_one_arg(mut args: Vec<Object>, name: &str) -> Result<Object, Object> {
    if args.len() != 1 {
        return Err(Object::error(format!("{name} expects exactly 1 argument")));
    }
    Ok(args.pop().unwrap())
}

fn expect_two_args(mut args: Vec<Object>, name: &str) -> Result<(Object, Object), Object> {
    if args.len() != 2 {
        return Err(Object::error(format!(
            "{name} expects exactly 2 arguments"
        )));
    }
    let b = args.pop().unwrap();
    let a = args.pop().unwrap();
    Ok((a, b))
}

pub(crate) fn math_abs(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "Math::abs") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::Integer(i) => Object::Integer(i.abs()),
        Object::Float(f) => Object::Float(f.abs()),
        other => Object::error(format!(
            "Math::abs expects integer or float, got {:?}",
            other
        )),
    }
}

pub(crate) fn math_floor(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "Math::floor") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::Integer(i) => Object::Integer(i),
        Object::Float(f) => Object::Integer(f.floor() as i64),
        other => Object::error(format!(
            "Math::floor expects integer or float, got {:?}",
            other
        )),
    }
}

pub(crate) fn math_ceil(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "Math::ceil") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::Integer(i) => Object::Integer(i),
        Object::Float(f) => Object::Integer(f.ceil() as i64),
        other => Object::error(format!(
            "Math::ceil expects integer or float, got {:?}",
            other
        )),
    }
}

pub(crate) fn math_round(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "Math::round") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::Integer(i) => Object::Integer(i),
        Object::Float(f) => Object::Integer(f.round() as i64),
        other => Object::error(format!(
            "Math::round expects integer or float, got {:?}",
            other
        )),
    }
}

pub(crate) fn math_min(args: Vec<Object>, _env: EnvRef) -> Object {
    let (a, b) = match expect_two_args(args, "Math::min") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match (a, b) {
        (Object::Integer(x), Object::Integer(y)) => Object::Integer(std::cmp::min(x, y)),
        (Object::Float(x), Object::Float(y)) => Object::Float(x.min(y)),
        (Object::Integer(x), Object::Float(y)) => Object::Float((x as f64).min(y)),
        (Object::Float(x), Object::Integer(y)) => Object::Float(x.min(y as f64)),
        (x, y) => Object::error(format!(
            "Math::min expects numeric arguments, got {:?} and {:?}",
            x, y
        )),
    }
}

pub(crate) fn math_max(args: Vec<Object>, _env: EnvRef) -> Object {
    let (a, b) = match expect_two_args(args, "Math::max") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match (a, b) {
        (Object::Integer(x), Object::Integer(y)) => Object::Integer(std::cmp::max(x, y)),
        (Object::Float(x), Object::Float(y)) => Object::Float(x.max(y)),
        (Object::Integer(x), Object::Float(y)) => Object::Float((x as f64).max(y)),
        (Object::Float(x), Object::Integer(y)) => Object::Float(x.max(y as f64)),
        (x, y) => Object::error(format!(
            "Math::max expects numeric arguments, got {:?} and {:?}",
            x, y
        )),
    }
}

pub(crate) fn math_pow(args: Vec<Object>, _env: EnvRef) -> Object {
    let (base, exp) = match expect_two_args(args, "Math::pow") {
        Ok(v) => v,
        Err(e) => return e,
    };

    fn as_f64(o: Object) -> Option<f64> {
        match o {
            Object::Integer(i) => Some(i as f64),
            Object::Float(f) => Some(f),
            _ => None,
        }
    }

    let Some(b_f) = as_f64(base.clone()) else {
        return Object::error(format!(
            "Math::pow expects numeric base, got {:?}",
            base
        ));
    };
    let Some(e_f) = as_f64(exp.clone()) else {
        return Object::error(format!(
            "Math::pow expects numeric exponent, got {:?}",
            exp
        ));
    };

    Object::Float(b_f.powf(e_f))
}

fn unary_f64(args: Vec<Object>, name: &str, f: fn(f64) -> f64) -> Object {
    let x = match expect_one_arg(args, name) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let val = match x {
        Object::Integer(i) => i as f64,
        Object::Float(fl) => fl,
        other => {
            return Object::error(format!(
                "{name} expects numeric argument, got {:?}",
                other
            ))
        }
    };

    Object::Float(f(val))
}

pub(crate) fn math_sin(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::sin", f64::sin)
}

pub(crate) fn math_cos(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::cos", f64::cos)
}

pub(crate) fn math_tan(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::tan", f64::tan)
}

pub(crate) fn math_sqrt(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::sqrt", f64::sqrt)
}

// Logarithmic functions

pub(crate) fn math_log(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::log", f64::ln)
}

pub(crate) fn math_log10(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::log10", f64::log10)
}

pub(crate) fn math_log2(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::log2", f64::log2)
}

pub(crate) fn math_exp(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::exp", f64::exp)
}

// Inverse trigonometric functions

pub(crate) fn math_asin(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::asin", f64::asin)
}

pub(crate) fn math_acos(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::acos", f64::acos)
}

pub(crate) fn math_atan(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::atan", f64::atan)
}

/// Math::atan2(y, x) - returns the angle in radians between the positive x-axis and the point (x, y)
pub(crate) fn math_atan2(args: Vec<Object>, _env: EnvRef) -> Object {
    let (y, x) = match expect_two_args(args, "Math::atan2") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let y_val = match y {
        Object::Integer(i) => i as f64,
        Object::Float(f) => f,
        other => {
            return Object::error(format!(
                "Math::atan2 expects numeric first argument, got {:?}",
                other
            ))
        }
    };

    let x_val = match x {
        Object::Integer(i) => i as f64,
        Object::Float(f) => f,
        other => {
            return Object::error(format!(
                "Math::atan2 expects numeric second argument, got {:?}",
                other
            ))
        }
    };

    Object::Float(y_val.atan2(x_val))
}

// Hyperbolic functions

pub(crate) fn math_sinh(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::sinh", f64::sinh)
}

pub(crate) fn math_cosh(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::cosh", f64::cosh)
}

pub(crate) fn math_tanh(args: Vec<Object>, _env: EnvRef) -> Object {
    unary_f64(args, "Math::tanh", f64::tanh)
}

// Mathematical constants

/// Math::PI() -> π ≈ 3.14159...
pub(crate) fn math_pi(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Math::PI expects no arguments");
    }
    Object::Float(std::f64::consts::PI)
}

/// Math::E() -> e ≈ 2.71828...
pub(crate) fn math_e(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Math::E expects no arguments");
    }
    Object::Float(std::f64::consts::E)
}

/// Math::TAU() -> τ = 2π ≈ 6.28318...
pub(crate) fn math_tau(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Math::TAU expects no arguments");
    }
    Object::Float(std::f64::consts::TAU)
}

// Utility functions

/// Math::sign(x) -> -1, 0, or 1
pub(crate) fn math_sign(args: Vec<Object>, _env: EnvRef) -> Object {
    let x = match expect_one_arg(args, "Math::sign") {
        Ok(v) => v,
        Err(e) => return e,
    };

    match x {
        Object::Integer(i) => {
            Object::Integer(if i < 0 { -1 } else if i > 0 { 1 } else { 0 })
        }
        Object::Float(f) => {
            if f.is_nan() {
                Object::Float(f64::NAN)
            } else if f < 0.0 {
                Object::Integer(-1)
            } else if f > 0.0 {
                Object::Integer(1)
            } else {
                Object::Integer(0)
            }
        }
        other => Object::error(format!(
            "Math::sign expects numeric argument, got {:?}",
            other
        )),
    }
}

fn expect_three_args(mut args: Vec<Object>, name: &str) -> Result<(Object, Object, Object), Object> {
    if args.len() != 3 {
        return Err(Object::error(format!(
            "{name} expects exactly 3 arguments"
        )));
    }
    let c = args.pop().unwrap();
    let b = args.pop().unwrap();
    let a = args.pop().unwrap();
    Ok((a, b, c))
}

/// Math::clamp(x, min, max) -> x clamped to [min, max]
pub(crate) fn math_clamp(args: Vec<Object>, _env: EnvRef) -> Object {
    let (x, min_val, max_val) = match expect_three_args(args, "Math::clamp") {
        Ok(v) => v,
        Err(e) => return e,
    };

    fn as_f64(o: &Object) -> Option<f64> {
        match o {
            Object::Integer(i) => Some(*i as f64),
            Object::Float(f) => Some(*f),
            _ => None,
        }
    }

    let x_f = match as_f64(&x) {
        Some(v) => v,
        None => {
            return Object::error(format!(
                "Math::clamp expects numeric first argument, got {:?}",
                x
            ))
        }
    };

    let min_f = match as_f64(&min_val) {
        Some(v) => v,
        None => {
            return Object::error(format!(
                "Math::clamp expects numeric second argument, got {:?}",
                min_val
            ))
        }
    };

    let max_f = match as_f64(&max_val) {
        Some(v) => v,
        None => {
            return Object::error(format!(
                "Math::clamp expects numeric third argument, got {:?}",
                max_val
            ))
        }
    };

    let result = x_f.max(min_f).min(max_f);

    // If all inputs were integers and result is whole, return integer
    if matches!(x, Object::Integer(_))
        && matches!(min_val, Object::Integer(_))
        && matches!(max_val, Object::Integer(_))
    {
        Object::Integer(result as i64)
    } else {
        Object::Float(result)
    }
}

/// Math::random() -> random float in [0, 1)
pub(crate) fn math_random(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Math::random expects no arguments");
    }

    use std::time::{SystemTime, UNIX_EPOCH};

    // Simple LCG-based random using time as seed
    // This is not cryptographically secure but fine for basic usage
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // Use a simple hash-like transformation
    let mut x = seed;
    x = x.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
    x ^= x >> 17;
    x = x.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);

    // Convert to float in [0, 1)
    let result = (x as f64) / (u64::MAX as f64);
    Object::Float(result)
}

/// Math::randomInt(min, max) -> random integer in [min, max]
pub(crate) fn math_random_int(args: Vec<Object>, _env: EnvRef) -> Object {
    let (min_val, max_val) = match expect_two_args(args, "Math::randomInt") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let min = match min_val {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Math::randomInt expects integer min, got {:?}",
                other
            ))
        }
    };

    let max = match max_val {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Math::randomInt expects integer max, got {:?}",
                other
            ))
        }
    };

    if min > max {
        return Object::error("Math::randomInt: min must be <= max");
    }

    use std::time::{SystemTime, UNIX_EPOCH};

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut x = seed;
    x = x.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
    x ^= x >> 17;
    x = x.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);

    let range = (max - min + 1) as u64;
    let result = min + (x % range) as i64;

    Object::Integer(result)
}

