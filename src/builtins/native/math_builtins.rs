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


