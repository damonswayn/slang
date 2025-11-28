use std::rc::Rc;

use crate::ast::nodes::{InfixOp, PrefixExpression, PrefixOp};
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
        Expression::FunctionLiteral(fl) => eval_function_literal(fl, env),
        Expression::CallExpression(call) => eval_call_expression(call, env),
        Expression::ArrayLiteral(al) => eval_array_literal(al, env),
        Expression::IndexExpression(ix) => eval_index_expression(ix, env),
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
            if let Expression::Identifier(Identifier { value: name }) = &*infix.left {
                let value = eval_expression(&infix.right, Rc::clone(&env));
                env.borrow_mut().set(name.clone(), value.clone());
                return value;
            } else {
                return Object::error("invalid assignment target");
            }
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
    let right = eval_expression(&pe.right, Rc::clone(&env));

    match pe.operator {
        PrefixOp::Not => eval_bang_operator(right),
        PrefixOp::Negate => eval_minus_prefix(right),
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

fn eval_function_literal(fl: &FunctionLiteral, env: EnvRef) -> Object {
    Object::Function {
        params: fl.params.clone(),
        body: fl.body.clone(),
        env,
    }
}

fn eval_call_expression(call: &CallExpression, env: EnvRef) -> Object {
    let function = eval_expression(&call.function, Rc::clone(&env));
    let args: Vec<Object> = call
        .arguments
        .iter()
        .map(|arg| eval_expression(arg, Rc::clone(&env)))
        .collect();

    apply_function(function, args)
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { params, body, env } => {
            let extended = new_enclosed_env(env);

            {
                let mut inner = extended.borrow_mut();
                for (param, arg) in params.iter().zip(args.into_iter()) {
                    inner.set(param.value.clone(), arg);
                }
            }

            super::stmt::eval_block_statement(&body, extended)
        }
        Object::Builtin(f) => f(args),
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

fn eval_index_expression(ix: &IndexExpression, env: EnvRef) -> Object {
    let left = eval_expression(&ix.left, Rc::clone(&env));
    let index = eval_expression(&ix.index, Rc::clone(&env));

    match (left, index) {
        (Object::Array(arr), Object::Integer(i)) => eval_array_index(arr, i),
        (Object::Array(_), other) => {
            Object::error(format!("array index must be integer, got {:?}", other))
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

/// Truthiness rules for Slang values.
pub(super) fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(false) => false,
        Object::Null => false,
        _ => true, // everything else is truthy
    }
}


