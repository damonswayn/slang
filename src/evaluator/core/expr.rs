use std::rc::Rc;

use crate::ast::nodes::{
    InfixOp, ObjectLiteral, PrefixExpression, PrefixOp, PropertyAccess, PostfixExpression,
    PostfixOp,
};
use crate::ast::{
    ArrayLiteral, CallExpression, Expression, FunctionLiteral, Identifier, IndexExpression,
    InfixExpression,
};
use crate::{builtins, debug_log};
use crate::env::{EnvRef, new_enclosed_env};
use crate::object::Object;

use super::stmt::eval_if_expression;

/// Evaluate an expression node.
pub(super) fn eval_expression(expr: &Expression, env: EnvRef) -> Object {
    match expr {
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::IntegerLiteral(il) => Object::Integer(il.value),
        Expression::FloatLiteral(fl) => Object::Float(fl.value),
        Expression::BooleanLiteral(bl) => Object::Boolean(bl.value),
        Expression::StringLiteral(sl) => Object::String(sl.value.clone()),
        Expression::Infix(infix) => eval_infix_expression(infix, env),
        Expression::If(ifexpr) => eval_if_expression(ifexpr, env),
        Expression::Prefix(p) => eval_prefix_expression(p, env),
        Expression::Postfix(p) => eval_postfix_expression(p, env),
        Expression::FunctionLiteral(fl) => eval_function_literal(fl, env),
        Expression::CallExpression(call) => eval_call_expression(call, env),
        Expression::ArrayLiteral(al) => eval_array_literal(al, env),
        Expression::IndexExpression(ix) => eval_index_expression(ix, env),
        Expression::ObjectLiteral(ol) => eval_object_literal(ol, env),
        Expression::PropertyAccess(pa) => eval_property_access(pa, env),
    }
}

fn eval_identifier(ident: &Identifier, env: EnvRef) -> Object {
    debug_log!("eval_identifier: looking up '{}'", ident.value);

    let env_borrow = env.borrow();
    if let Some(val) = env_borrow.get(&ident.value) {
        debug_log!("  found in env: {:?}", val);
        return val;
    }

    if let Some(builtin_fn) = builtins::get(&ident.value) {
        debug_log!("  resolved as builtin");
        return Object::Builtin(builtin_fn);
    }

    debug_log!("  not found (returning Error)");
    Object::error(format!("identifier not found: {}", ident.value))
}

fn eval_infix_expression(infix: &InfixExpression, env: EnvRef) -> Object {
    let left = eval_expression(&infix.left, Rc::clone(&env));
    let right = eval_expression(&infix.right, Rc::clone(&env));

    use InfixOp::*;

    match infix.operator {
        Assign => {
            // Simple variable assignment: `x = expr`
            if let Expression::Identifier(Identifier { value: name }) = &*infix.left {
                let value = eval_expression(&infix.right, Rc::clone(&env));
                env.borrow_mut().set(name.clone(), value.clone());
                return value;
            }

            // Object property assignment: `obj.field = expr` or nested `obj.a.b = expr`
            if let Expression::PropertyAccess(pa) = &*infix.left {
                let value = eval_expression(&infix.right, Rc::clone(&env));
                let result = assign_to_property_access(pa, Rc::clone(&env), value.clone());
                return match result {
                    Ok(()) => value,
                    Err(msg) => Object::error(msg),
                };
            }

            // Object index assignment: `obj["field"] = expr` or nested `obj["a"]["b"] = expr`
            if let Expression::IndexExpression(_) = &*infix.left {
                let value = eval_expression(&infix.right, Rc::clone(&env));
                let result =
                    assign_to_index_expression(&infix.left, Rc::clone(&env), value.clone());
                return match result {
                    Ok(()) => value,
                    Err(msg) => Object::error(msg),
                };
            }

            return Object::error("invalid assignment target");
        }
        And => {
            let left = eval_expression(&infix.left, Rc::clone(&env));

            if !is_truthy(&left) {
                return Object::Boolean(false);
            }

            let right = eval_expression(&infix.right, Rc::clone(&env));
            return Object::Boolean(is_truthy(&right));
        }
        Or => {
            let left = eval_expression(&infix.left, Rc::clone(&env));

            if is_truthy(&left) {
                return Object::Boolean(true);
            }

            let right = eval_expression(&infix.right, Rc::clone(&env));
            return Object::Boolean(is_truthy(&right));
        }
        _ => {}
    }

    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(&infix.operator, l, r),
        (Object::Float(l), Object::Float(r)) => eval_float_infix(&infix.operator, l, r),

        // mixed numeric types are coerced to float, so we can use the same logic as for integers
        (Object::Integer(l), Object::Float(r)) => eval_float_infix(&infix.operator, l as f64, r),
        (Object::Float(l), Object::Integer(r)) => eval_float_infix(&infix.operator, l, r as f64),

        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix(&infix.operator, l, r),
        (Object::String(l), Object::String(r)) => eval_string_infix(&infix.operator, &l, &r),
        (l, r) => Object::error(format!(
            "type mismatch: {:?} {} {:?}",
            l, infix.operator, r
        )),
    }
}

fn eval_integer_infix(op: &InfixOp, left: i64, right: i64) -> Object {
    use InfixOp::*;
    match op {
        Plus => Object::Integer(left + right),
        Minus => Object::Integer(left - right),
        Multiply => Object::Integer(left * right),
        Divide => Object::Float(left as f64 / right as f64),
        Modulo => Object::Float(left as f64 % right as f64),

        LessThan => Object::Boolean(left < right),
        LessEqual => Object::Boolean(left <= right),
        GreaterThan => Object::Boolean(left > right),
        GreaterEqual => Object::Boolean(left >= right),
        Equals => Object::Boolean(left == right),
        NotEquals => Object::Boolean(left != right),
        _ => Object::error(format!("unknown operator: {} (integers)", op)),
    }
}

fn eval_float_infix(op: &InfixOp, left: f64, right: f64) -> Object {
    use InfixOp::*;
    match op {
        Plus => Object::Float(left + right),
        Minus => Object::Float(left - right),
        Multiply => Object::Float(left * right),
        Divide => Object::Float(left / right),
        Modulo => Object::Float(left % right),

        LessThan => Object::Boolean(left < right),
        LessEqual => Object::Boolean(left <= right),
        GreaterThan => Object::Boolean(left > right),
        GreaterEqual => Object::Boolean(left >= right),
        Equals => Object::Boolean(left == right),
        NotEquals => Object::Boolean(left != right),
        _ => Object::error(format!("unknown operator: {} (floats)", op)),
    }
}

fn eval_boolean_infix(op: &InfixOp, left: bool, right: bool) -> Object {
    use InfixOp::*;
    match op {
        Equals => Object::Boolean(left == right),
        NotEquals => Object::Boolean(left != right),
        _ => Object::error(format!("unknown operator: {} (booleans)", op)),
    }
}

fn eval_prefix_expression(pe: &PrefixExpression, env: EnvRef) -> Object {
    use PrefixOp::*;

    match pe.operator {
        Not => {
            let right = eval_expression(&pe.right, Rc::clone(&env));
            eval_bang_operator(right)
        }
        Negate => {
            let right = eval_expression(&pe.right, Rc::clone(&env));
            eval_minus_prefix(right)
        }
        PreIncrement => eval_inc_dec_expression(&pe.right, Rc::clone(&env), true, true),
        PreDecrement => eval_inc_dec_expression(&pe.right, Rc::clone(&env), false, true),
    }
}

fn eval_postfix_expression(pe: &PostfixExpression, env: EnvRef) -> Object {
    match pe.operator {
        PostfixOp::Increment => eval_inc_dec_expression(&pe.left, env, true, false),
        PostfixOp::Decrement => eval_inc_dec_expression(&pe.left, env, false, false),
    }
}

fn eval_bang_operator(obj: Object) -> Object {
    match obj {
        Object::Boolean(b) => Object::Boolean(!b),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix(obj: Object) -> Object {
    match obj {
        Object::Integer(i) => Object::Integer(-i),
        Object::Float(f) => Object::Float(-f),
        _ => Object::Null,
    }
}

fn eval_inc_dec_expression(
    target: &Expression,
    env: EnvRef,
    is_increment: bool,
    is_prefix: bool,
) -> Object {
    match target {
        Expression::Identifier(ident) => apply_inc_dec_to_identifier(ident, env, is_increment, is_prefix),
        Expression::PropertyAccess(pa) => apply_inc_dec_to_property(pa, env, is_increment, is_prefix),
        _ => Object::error("invalid increment/decrement target"),
    }
}

fn apply_inc_dec_to_identifier(
    ident: &Identifier,
    env: EnvRef,
    is_increment: bool,
    is_prefix: bool,
) -> Object {
    let current = eval_identifier(ident, Rc::clone(&env));
    if current.is_error() {
        return current;
    }

    let new_value = match apply_inc_dec_to_numeric(&current, is_increment) {
        Ok(v) => v,
        Err(msg) => return Object::error(msg),
    };

    env.borrow_mut().set(ident.value.clone(), new_value.clone());

    if is_prefix {
        new_value
    } else {
        current
    }
}

fn apply_inc_dec_to_property(
    pa: &PropertyAccess,
    env: EnvRef,
    is_increment: bool,
    is_prefix: bool,
) -> Object {
    let current = eval_property_access(pa, Rc::clone(&env));
    if current.is_error() {
        return current;
    }

    let new_value = match apply_inc_dec_to_numeric(&current, is_increment) {
        Ok(v) => v,
        Err(msg) => return Object::error(msg),
    };

    let result = assign_to_property_access(pa, Rc::clone(&env), new_value.clone());
    if let Err(msg) = result {
        return Object::error(msg);
    }

    if is_prefix {
        new_value
    } else {
        current
    }
}

fn apply_inc_dec_to_numeric(value: &Object, is_increment: bool) -> Result<Object, String> {
    match value {
        Object::Integer(i) => {
            let delta = if is_increment { 1 } else { -1 };
            Ok(Object::Integer(i + delta))
        }
        Object::Float(f) => {
            let delta = if is_increment { 1.0 } else { -1.0 };
            Ok(Object::Float(f + delta))
        }
        other => Err(format!(
            "increment/decrement only supported on numeric values, got {:?}",
            other
        )),
    }
}

fn eval_function_literal(fl: &FunctionLiteral, env: EnvRef) -> Object {
    Object::Function {
        params: fl.params.clone(),
        body: fl.body.clone(),
        env,
    }
}

fn eval_call_expression(call: &CallExpression, env: EnvRef) -> Object {
    // Evaluate arguments first (left-to-right)
    let args: Vec<Object> = call
        .arguments
        .iter()
        .map(|arg| eval_expression(arg, Rc::clone(&env)))
        .collect();

    // Special-case method calls: `obj.method(...)`
    if let Expression::PropertyAccess(pa) = &*call.function {
        let receiver = eval_expression(&pa.object, Rc::clone(&env));
        if receiver.is_error() {
            return receiver;
        }

        let method = match &receiver {
            Object::Object(map) => map
                .get(&pa.property.value)
                .cloned()
                .unwrap_or(Object::Null),
            other => {
                return Object::error(format!(
                    "property call not supported on value: {:?}",
                    other
                ))
            }
        };

        return apply_function_with_this(method, args, Some(receiver), Rc::clone(&env));
    }

    // Regular function call
    let function = eval_expression(&call.function, Rc::clone(&env));
    apply_function_with_this(function, args, None, env)
}

/// Apply a function or builtin value to arguments, optionally binding `this`
/// for method-style calls. Exposed so native builtins can reuse the same
/// calling convention when they receive higher-order function arguments.
pub fn apply_function_with_this(
    func: Object,
    args: Vec<Object>,
    this: Option<Object>,
    caller_env: EnvRef,
) -> Object {
    match func {
        Object::Function { params, body, env } => {
            let extended = new_enclosed_env(env);

            {
                let mut inner = extended.borrow_mut();

                // Bind implicit `this` for method calls, if provided.
                if let Some(this_val) = this {
                    inner.set("this".to_string(), this_val);
                }

                for (param, arg) in params.iter().zip(args.into_iter()) {
                    inner.set(param.value.clone(), arg);
                }
            }

            // Execute function body and unwrap an explicit `return` value if present,
            // so callers see the inner value rather than a ReturnValue wrapper.
            let result = super::stmt::eval_block_statement(&body, extended);
            if let Object::ReturnValue(inner) = result {
                *inner
            } else {
                result
            }
        }
        Object::Builtin(f) => f(args, caller_env),
        other => Object::error(format!("not a function: {:?}", other)),
    }
}

fn eval_string_infix(op: &InfixOp, left: &str, right: &str) -> Object {
    use InfixOp::*;
    match op {
        Plus => {
            let mut s = String::with_capacity(left.len() + right.len());
            s.push_str(left);
            s.push_str(right);
            Object::String(s)
        }
        Equals => Object::Boolean(left == right),
        NotEquals => Object::Boolean(left != right),
        _ => Object::error(format!("unknown operator: {} (strings)", op)),
    }
}

fn eval_array_literal(al: &ArrayLiteral, env: EnvRef) -> Object {
    let elements = al
        .elements
        .iter()
        .map(|e| eval_expression(e, Rc::clone(&env)))
        .collect::<Vec<_>>();
    Object::Array(elements)
}

fn eval_object_literal(ol: &ObjectLiteral, env: EnvRef) -> Object {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for (ident, expr) in &ol.properties {
        let value = eval_expression(expr, Rc::clone(&env));
        if value.is_error() {
            return value;
        }
        map.insert(ident.value.clone(), value);
    }

    Object::Object(map)
}

fn eval_index_expression(ix: &IndexExpression, env: EnvRef) -> Object {
    let left = eval_expression(&ix.left, Rc::clone(&env));
    let index = eval_expression(&ix.index, Rc::clone(&env));

    match (left, index) {
        (Object::Array(arr), Object::Integer(i)) => eval_array_index(arr, i),
        (Object::Array(_), other) => {
            Object::error(format!("array index must be integer, got {:?}", other))
        }
        (Object::Object(map), Object::String(key)) => {
            map.get(&key).cloned().unwrap_or(Object::Null)
        }
        (Object::Object(_), other) => {
            Object::error(format!("object index must be string, got {:?}", other))
        }
        (other, _) => Object::error(format!("index operator not supported: {:?}", other)),
    }
}

fn eval_array_index(arr: Vec<Object>, index: i64) -> Object {
    if index < 0 {
        return Object::Null;
    }

    let idx = index as usize;
    if idx >= arr.len() {
        Object::Null
    } else {
        arr[idx].clone()
    }
}

fn eval_property_access(pa: &PropertyAccess, env: EnvRef) -> Object {
    let obj = eval_expression(&pa.object, Rc::clone(&env));
    if obj.is_error() {
        return obj;
    }

    match obj {
        Object::Object(map) => map
            .get(&pa.property.value)
            .cloned()
            .unwrap_or(Object::Null),
        other => Object::error(format!(
            "property access not supported on value: {:?}",
            other
        )),
    }
}

/// Handle assignments like `obj.field = value` and nested `obj.a.b = value`.
fn assign_to_property_access(
    pa: &PropertyAccess,
    env: EnvRef,
    new_value: Object,
) -> Result<(), String> {
    // Collect property chain from the AST, e.g. for `obj.a.b` we get:
    //   root_ident = "obj", props = ["a", "b"]
    let mut props: Vec<String> = vec![pa.property.value.clone()];
    let root_ident = match collect_property_chain(&pa.object, &mut props) {
        Some(name) => name,
        None => {
            return Err("left side of assignment must be an object property (like x.y or x.y.z)"
                .to_string())
        }
    };

    debug_log!(
        "assign_to_property_access: root = {}, props = {:?}",
        root_ident,
        props
    );

    // Get current root object value from environment
    let current_root = {
        let env_borrow = env.borrow();
        match env_borrow.get(&root_ident) {
            Some(obj) => obj,
            None => {
                return Err(format!(
                    "identifier not found for property assignment: {}",
                    root_ident
                ))
            }
        }
    };

    // Recursively build an updated root object with the new value applied
    let updated_root =
        assign_into_object(current_root.clone(), &props, &new_value).map_err(|e| e)?;

    // Store updated root back into current environment scope
    env.borrow_mut()
        .set(root_ident, updated_root);

    Ok(())
}

/// Walks back through nested `PropertyAccess` to find the root identifier and
/// complete property chain.
fn collect_property_chain(expr: &Expression, props: &mut Vec<String>) -> Option<String> {
    match expr {
        Expression::Identifier(Identifier { value }) => {
            // We have reached the root: reverse the collected props so they are
            // in left-to-right order (outermost to innermost).
            props.reverse();
            Some(value.clone())
        }
        Expression::PropertyAccess(inner) => {
            props.push(inner.property.value.clone());
            collect_property_chain(&inner.object, props)
        }
        _ => None,
    }
}

/// Handle assignments like `obj["field"] = value` and nested `obj["a"]["b"] = value`.
fn assign_to_index_expression(
    target: &Expression,
    env: EnvRef,
    new_value: Object,
) -> Result<(), String> {
    let mut props: Vec<String> = Vec::new();
    let root_ident =
        collect_index_chain(target, Rc::clone(&env), &mut props).ok_or_else(|| {
            "left side of assignment must be an object index (like x[\"y\"] or x[\"a\"][\"b\"])"
                .to_string()
        })?;

    debug_log!(
        "assign_to_index_expression: root = {}, props = {:?}",
        root_ident,
        props
    );

    // Get current root object value from environment
    let current_root = {
        let env_borrow = env.borrow();
        match env_borrow.get(&root_ident) {
            Some(obj) => obj,
            None => {
                return Err(format!(
                    "identifier not found for index assignment: {}",
                    root_ident
                ))
            }
        }
    };

    // Recursively build an updated root object with the new value applied
    let updated_root =
        assign_into_object(current_root.clone(), &props, &new_value).map_err(|e| e)?;

    // Store updated root back into current environment scope
    env.borrow_mut().set(root_ident, updated_root);

    Ok(())
}

/// Collect the root identifier and dynamic string key chain for an index-based
/// assignment target, e.g. `obj["a"]["b"]` -> root `obj`, props = ["a", "b"].
fn collect_index_chain(
    expr: &Expression,
    env: EnvRef,
    props: &mut Vec<String>,
) -> Option<String> {
    match expr {
        Expression::IndexExpression(ix) => {
            // Evaluate the index expression and ensure it is a string key.
            let key_val = eval_expression(&ix.index, Rc::clone(&env));
            match key_val {
                Object::String(s) => props.push(s),
                _ => {
                    return None; // will be turned into a user-facing error by caller
                }
            }

            collect_index_chain(&ix.left, env, props)
        }
        Expression::Identifier(Identifier { value }) => {
            // We have reached the root identifier; reverse collected props to
            // be in left-to-right order (outermost to innermost).
            props.reverse();
            Some(value.clone())
        }
        _ => None,
    }
}

/// Given a root object and a property path, produces a new value with the
/// property updated, preserving value semantics.
fn assign_into_object(
    obj: Object,
    props: &[String],
    new_value: &Object,
) -> Result<Object, String> {
    if props.is_empty() {
        return Ok(new_value.clone());
    }

    match obj {
        Object::Object(mut map) => {
            let key = &props[0];

            if props.len() == 1 {
                // Final property: just insert / overwrite
                map.insert(key.clone(), new_value.clone());
                Ok(Object::Object(map))
            } else {
                // Need to drill down into nested object
                let child = map.remove(key).unwrap_or_else(|| Object::Object(Default::default()));

                let updated_child = assign_into_object(child, &props[1..], new_value)?;
                map.insert(key.clone(), updated_child);
                Ok(Object::Object(map))
            }
        }
        other => Err(format!(
            "cannot assign property on non-object value: {:?}",
            other
        )),
    }
}

/// Truthiness rules for Slang values.
pub(super) fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(false) => false,
        Object::Null => false,
        _ => true, // everything else is truthy
    }
}


