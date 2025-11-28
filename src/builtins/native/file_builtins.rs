use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::rc::Rc;
use crate::object::Object;
use crate::object::types::{FileHandle, FileRef};

// Builtin functions

pub fn builtin_open(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("wrong number of arguments".into());
    }

    let path = match &args[0] {
        Object::String(s) => s.clone(),
        _ => return Object::Error("expected string argument".into()),
    };

    let mode = match &args[1] {
        Object::String(s) => s.clone(),
        _ => return Object::Error("expected string argument".into()),
    };

    let mut opts = OpenOptions::new();
    match mode.as_ref() {
        "r" => { opts.read(true); },
        "w" => { opts.write(true).create(true).truncate(true); },
        "a" => { opts.append(true).create(true); },
        "r+" => { opts.read(true).write(true); },
        "w+" => { opts.write(true).create(true).truncate(true).read(true); },
        "a+" => { opts.append(true).create(true).read(true); },
        _ => return Object::Error("invalid mode".into()),
    }

    match opts.open(path) {
        Ok(file) => Object::File(Rc::new(RefCell::new(FileHandle::new(file)))),
        Err(e) => Object::Error(format!("failed to open file: {}", e))
    }
}

pub fn builtin_read(args: Vec<Object>) -> Object {
    if args.len() < 1 || args.len() > 2 {
        return Object::Error("wrong number of arguments".into());
    }

    let file_reference = match expect_file(&args[0]) {
        Ok(file_reference) => file_reference,
        Err(e) => return e,
    };

    let mut guard = file_reference.borrow_mut();
    let file = match guard.inner.as_mut() {
        Some(f) => f,
        None => return Object::Error("file is already closed".into()),
    };

    let mut buf = Vec::new();
    if args.len() == 2 {
        let n = match &args[1] {
            Object::Integer(n) => *n,
            _ => return Object::Error("expected integer argument".into()),
        };

        if n < 0 {
            return Object::Error("number of bytes to read must be >= 0".into());
        }

        let mut chunk = vec![0u8; n as usize];
        match file.read(&mut chunk) {
            Ok(read) => {
                chunk.truncate(read);
                match String::from_utf8(chunk) {
                    Ok(s) => Object::String(s),
                    Err(e) => Object::Error(format!("failed to decode UTF-8: {}", e)),
                }
            },
            Err(e) => Object::Error(format!("failed to read from file: {}", e)),
        }
    } else {
        match file.read_to_end(&mut buf) {
            Ok(_) => match String::from_utf8(buf) {
                Ok(s) => Object::String(s),
                Err(e) => Object::Error(format!("failed to decode UTF-8: {}", e)),
            },
            Err(e) => Object::Error(format!("failed to read from file: {}", e)),
        }
    }
}

pub fn builtin_write(args: Vec<Object>) -> Object {
    if args.len() != 2 { return Object::Error("write(file, data) expects 2 args".into()) }
    let file_reference = match expect_file(&args[0]) { Ok(f) => f, Err(e) => return e };
    let data = match &args[1] { Object::String(s) => s.clone(), _ => return Object::Error("write: data must be string".into()) };

    let mut guard = file_reference.borrow_mut();
    let file = match guard.inner.as_mut() { Some(f) => f, None => return Object::Error("write: file is closed".into()) };

    match file.write(data.as_bytes()) {
        Ok(w) => Object::Integer(w as i64),
        Err(e) => Object::Error(format!("write: {}", e)),
    }
}

pub fn builtin_seek(args: Vec<Object>) -> Object {
    if args.len() != 3 { return Object::Error("seek(file, offset, whence) expects 3 args".into()) }
    let file_reference = match expect_file(&args[0]) { Ok(f) => f, Err(e) => return e };
    let offset = match &args[1] { Object::Integer(i) => *i, _ => return Object::Error("seek: offset must be integer".into()) };
    let whence = match &args[2] { Object::String(s) => s.as_str(), _ => return Object::Error("seek: whence must be string".into()) };

    let mut guard = file_reference.borrow_mut();
    let file = match guard.inner.as_mut() { Some(f) => f, None => return Object::Error("seek: file is closed".into()) };

    let seek_from = match whence {
        "start" => SeekFrom::Start(offset as u64),
        "current" => SeekFrom::Current(offset),
        "end" => SeekFrom::End(offset),
        _ => return Object::Error("seek: whence must be 'start'|'current'|'end'".into()),
    };

    match file.seek(seek_from) {
        Ok(w) => Object::Integer(w as i64),
        Err(e) => Object::Error(format!("seek: {}", e)),
    }
}

pub fn builtin_close(args: Vec<Object>) -> Object {
    if args.len() != 1 { return Object::Error("close(file) expects 1 arg".into()) }
    let file_reference = match &args[0] {
        Object::File(fr) => Rc::clone(fr),
        _ => return Object::Error("close: expected file".into()),
    };

    let mut guard = file_reference.borrow_mut();
    guard.inner = None; // drop the File => closes it
    Object::Null
}

// Helpers

fn expect_file(obj: &Object) -> Result<FileRef, Object> {
    if let Object::File(file_reference) = obj {
        if file_reference.borrow().is_closed() {
            Err(Object::Error("file is closed".into()))
        } else {
            Ok(Rc::clone(file_reference))
        }
    } else {
        Err(Object::Error("expected file object".into()))
    }
}