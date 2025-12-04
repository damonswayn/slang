use std::rc::Rc;

use crate::ast::nodes::{ForStatement, FunctionStatement, ReturnStatement, TestStatement};
use crate::ast::{
    BlockStatement, IfExpression, LetStatement, Statement, WhileStatement,
};
use crate::env::EnvRef;
use crate::object::Object;

use super::expr::{eval_expression, is_truthy};

pub(super) fn eval_statement(stmt: &Statement, env: EnvRef) -> Object {
    match stmt {
        Statement::Let(ls) => eval_let_statement(ls, Rc::clone(&env)),
        Statement::Return(rs) => eval_return_statement(rs, Rc::clone(&env)),
        Statement::While(ws) => eval_while_statement(ws, Rc::clone(&env)),
        Statement::For(fs) => eval_for_statement(fs, Rc::clone(&env)),
        Statement::Expression(es) => eval_expression(&es.expression, Rc::clone(&env)),
        Statement::Function(fs) => eval_function_statement(fs, Rc::clone(&env)),
        Statement::Test(ts) => eval_test_statement(ts, Rc::clone(&env)),
    }
}

fn eval_let_statement(ls: &LetStatement, env: EnvRef) -> Object {
    let val = eval_expression(&ls.value, Rc::clone(&env));
    env.borrow_mut().set(ls.name.value.clone(), val.clone());
    // let itself doesn't produce a useful value
    Object::Null
}

pub(super) fn eval_block_statement(block: &BlockStatement, env: EnvRef) -> Object {
    let mut result = Object::Null;

    for stmt in &block.statements {
        result = eval_statement(stmt, Rc::clone(&env));

        if let Object::ReturnValue(_) = result {
            return result;
        }
        if result.is_error() {
            return result;
        }
    }

    result
}

pub(super) fn eval_if_expression(ifexpr: &IfExpression, env: EnvRef) -> Object {
    let condition = eval_expression(&ifexpr.condition, Rc::clone(&env));

    if is_truthy(&condition) {
        eval_block_statement(&ifexpr.consequence, Rc::clone(&env))
    } else if let Some(alt) = &ifexpr.alternative {
        eval_block_statement(alt, Rc::clone(&env))
    } else {
        Object::Null
    }
}

fn eval_return_statement(rs: &ReturnStatement, env: EnvRef) -> Object {
    let val = eval_expression(&rs.return_value, Rc::clone(&env));
    Object::ReturnValue(Box::new(val))
}

fn eval_while_statement(ws: &WhileStatement, env: EnvRef) -> Object {
    let mut result = Object::Null;

    loop {
        let cond = eval_expression(&ws.condition, Rc::clone(&env));
        if !is_truthy(&cond) {
            break;
        }

        result = eval_block_statement(&ws.body, Rc::clone(&env));

        // propagate return out of the loop
        if let Object::ReturnValue(_) = result {
            return result;
        }
    }

    result
}

fn eval_for_statement(fs: &ForStatement, env: EnvRef) -> Object {
    // init
    if let Some(init_stmt) = &fs.init {
        let init_result = eval_statement(init_stmt, Rc::clone(&env));
        if let Object::ReturnValue(_) = init_result {
            return init_result;
        }
    }

    let mut result = Object::Null;

    loop {
        // condition
        if let Some(cond_expr) = &fs.condition {
            let cond = eval_expression(cond_expr, Rc::clone(&env));
            if !is_truthy(&cond) {
                break;
            }
        }

        // body
        result = eval_block_statement(&fs.body, Rc::clone(&env));
        if let Object::ReturnValue(_) = result {
            return result;
        }

        // post
        if let Some(post_stmt) = &fs.post {
            let post_result = eval_statement(post_stmt, Rc::clone(&env));
            if let Object::ReturnValue(_) = post_result {
                return post_result;
            }
        }
    }

    result
}

fn eval_function_statement(fs: &FunctionStatement, env: EnvRef) -> Object {
    // Build the same Object::Function your eval_function_literal creates
    let func_obj = Object::Function {
        params: fs.literal.params.clone(),
        body: fs.literal.body.clone(),
        env: Rc::clone(&env), // capture defining env for closures/recursion
    };

    env.borrow_mut().set(fs.name.value.clone(), func_obj);

    // Like 'let', defining a function doesn't produce a value
    Object::Null
}

fn eval_test_statement(_ts: &TestStatement, _env: EnvRef) -> Object {
    // Test blocks are only executed by the dedicated test runner mode.
    // In regular script evaluation they are treated as no-ops so that
    // scripts containing tests can still be run normally.
    Object::Null
}


