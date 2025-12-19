use std::collections::HashMap;
use std::env;
use std::process::{Command, exit};

use crate::env::EnvRef;
use crate::object::Object;

fn expect_one_arg(mut args: Vec<Object>, name: &str) -> Result<Object, Object> {
    if args.len() != 1 {
        return Err(Object::error(format!("{name} expects exactly 1 argument")));
    }
    Ok(args.pop().unwrap())
}

/// Sys::env() -> object (all environment variables)
/// Sys::env(name) -> Option (specific environment variable)
pub(crate) fn sys_env(args: Vec<Object>, _env: EnvRef) -> Object {
    if args.is_empty() {
        // Return all environment variables as an object
        let mut map = HashMap::new();
        for (key, value) in env::vars() {
            map.insert(key, Object::String(value));
        }
        return Object::Object(map);
    }

    if args.len() != 1 {
        return Object::error("Sys::env expects 0 or 1 arguments");
    }

    let name = match &args[0] {
        Object::String(s) => s.clone(),
        other => {
            return Object::error(format!(
                "Sys::env expects string variable name, got {:?}",
                other
            ))
        }
    };

    match env::var(&name) {
        Ok(value) => Object::OptionSome(Box::new(Object::String(value))),
        Err(_) => Object::OptionNone,
    }
}

/// Sys::setEnv(name, value) -> null (sets an environment variable)
pub(crate) fn sys_set_env(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Sys::setEnv expects exactly 2 arguments (name, value)");
    }

    let value = args.pop().unwrap();
    let name = args.pop().unwrap();

    let name_str = match name {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Sys::setEnv expects string name, got {:?}",
                other
            ))
        }
    };

    let value_str = match value {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Sys::setEnv expects string value, got {:?}",
                other
            ))
        }
    };

    // SAFETY: This is only safe in single-threaded contexts or when no other
    // threads are reading this environment variable.
    unsafe {
        env::set_var(&name_str, &value_str);
    }
    Object::Null
}

/// Sys::args() -> array (command line arguments)
pub(crate) fn sys_args(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Sys::args expects no arguments");
    }

    let args: Vec<Object> = env::args().map(|a| Object::String(a)).collect();
    Object::Array(args)
}

/// Sys::exit(code) -> never returns (exits the process)
pub(crate) fn sys_exit(args: Vec<Object>, _env: EnvRef) -> Object {
    let code = match expect_one_arg(args, "Sys::exit") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let code_val = match code {
        Object::Integer(i) => i as i32,
        other => {
            return Object::error(format!(
                "Sys::exit expects integer exit code, got {:?}",
                other
            ))
        }
    };

    exit(code_val);
}

/// Sys::cwd() -> string (current working directory)
pub(crate) fn sys_cwd(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Sys::cwd expects no arguments");
    }

    match env::current_dir() {
        Ok(path) => Object::String(path.to_string_lossy().to_string()),
        Err(e) => Object::error(format!("Failed to get current directory: {}", e)),
    }
}

/// Sys::setCwd(path) -> Result (changes the current working directory)
pub(crate) fn sys_set_cwd(args: Vec<Object>, _env: EnvRef) -> Object {
    let path = match expect_one_arg(args, "Sys::setCwd") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let path_str = match path {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Sys::setCwd expects string path, got {:?}",
                other
            ))
        }
    };

    match env::set_current_dir(&path_str) {
        Ok(_) => Object::ResultOk(Box::new(Object::Null)),
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "Failed to change directory: {}",
            e
        )))),
    }
}

/// Sys::exec(command) -> Result({ stdout, stderr, code })
/// Executes a shell command and returns the result
pub(crate) fn sys_exec(args: Vec<Object>, _env: EnvRef) -> Object {
    let cmd = match expect_one_arg(args, "Sys::exec") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let cmd_str = match cmd {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Sys::exec expects string command, got {:?}",
                other
            ))
        }
    };

    // Use shell to execute the command
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &cmd_str]).output()
    } else {
        Command::new("sh").args(["-c", &cmd_str]).output()
    };

    match output {
        Ok(result) => {
            let mut map = HashMap::new();
            map.insert(
                "stdout".to_string(),
                Object::String(String::from_utf8_lossy(&result.stdout).to_string()),
            );
            map.insert(
                "stderr".to_string(),
                Object::String(String::from_utf8_lossy(&result.stderr).to_string()),
            );
            map.insert(
                "code".to_string(),
                Object::Integer(result.status.code().unwrap_or(-1) as i64),
            );

            if result.status.success() {
                Object::ResultOk(Box::new(Object::Object(map)))
            } else {
                Object::ResultErr(Box::new(Object::Object(map)))
            }
        }
        Err(e) => Object::ResultErr(Box::new(Object::String(format!(
            "Failed to execute command: {}",
            e
        )))),
    }
}

/// Sys::platform() -> string (operating system name)
pub(crate) fn sys_platform(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Sys::platform expects no arguments");
    }

    Object::String(env::consts::OS.to_string())
}

/// Sys::arch() -> string (CPU architecture)
pub(crate) fn sys_arch(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Sys::arch expects no arguments");
    }

    Object::String(env::consts::ARCH.to_string())
}

