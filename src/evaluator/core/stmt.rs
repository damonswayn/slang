use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::ast::nodes::{
    ForStatement, FunctionStatement, NamespaceStatement, ReturnStatement, TestStatement,
};
use crate::ast::{
    BlockStatement, IfExpression, ImportStatement, LetStatement, Statement, WhileStatement,
};
use crate::env::{EnvRef, new_enclosed_env, new_env};
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

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
        Statement::Namespace(ns) => eval_namespace_statement(ns, Rc::clone(&env)),
        Statement::Import(is) => eval_import_statement(is, Rc::clone(&env)),
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

fn eval_namespace_statement(ns: &NamespaceStatement, env: EnvRef) -> Object {
    // Evaluate within an enclosed environment to avoid leaking locals.
    let ns_env = new_enclosed_env(Rc::clone(&env));
    let result = eval_block_statement(&ns.body, Rc::clone(&ns_env));

    if result.is_error() {
        return result;
    }
    if let Object::ReturnValue(inner) = result {
        return *inner;
    }

    let exported = ns_env.borrow().snapshot();
    env.borrow_mut()
        .set(ns.name.value.clone(), Object::Object(exported));

    Object::Null
}

fn eval_import_statement(is: &ImportStatement, env: EnvRef) -> Object {
    let path = Path::new(&is.path);
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        let base_dir = env
            .borrow()
            .module_dir()
            .or_else(|| std::env::current_dir().ok());
        match base_dir {
            Some(base) => base.join(path),
            None => return Object::error("unable to resolve import: no base directory"),
        }
    };

    let source = match fs::read_to_string(&resolved) {
        Ok(s) => s,
        Err(err) => {
            return Object::error(format!(
                "failed to read import '{}': {}",
                resolved.display(),
                err
            ))
        }
    };

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        return Object::error(format!(
            "parse errors in import '{}': {:?}",
            resolved.display(),
            parser.errors
        ));
    }

    // Evaluate imported file in a fresh environment; only namespaces are exported.
    let module_env = new_env();
    let parent_dir = resolved.parent().map(|p| p.to_path_buf());
    module_env.borrow_mut().set_module_dir(parent_dir);
    let eval_result = crate::evaluator::eval(&program, Rc::clone(&module_env));
    if eval_result.is_error() {
        return eval_result;
    }

    let module_store = module_env.borrow().snapshot();
    for (name, value) in module_store {
        if is_builtin_namespace(&name) {
            continue;
        }

        if let Object::Object(ns_obj) = value {
            merge_namespace_into_env(&name, ns_obj, Rc::clone(&env));
        }
    }

    Object::Null
}

fn merge_namespace_into_env(
    name: &str,
    ns_obj: std::collections::HashMap<String, Object>,
    env: EnvRef,
) {
    let mut env_mut = env.borrow_mut();
    match env_mut.get(name) {
        Some(Object::Object(existing)) => {
            let mut merged = existing.clone();
            for (k, v) in ns_obj {
                merged.insert(k, v);
            }
            env_mut.set(name.to_string(), Object::Object(merged));
        }
        _ => {
            env_mut.set(name.to_string(), Object::Object(ns_obj));
        }
    }
}

fn is_builtin_namespace(name: &str) -> bool {
    matches!(
        name,
        "Option" | "Result" | "Regex" | "File" | "Array" | "Math" | "String" | "Json" | "Test"
    )
}


