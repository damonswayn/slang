use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::object::Object;
use crate::builtins::native::monad_builtins::{
    option_some,
    option_none,
    option_is_some,
    option_is_none,
    option_unwrap_or,
    option_map,
    option_and_then,
    option_bind,
    option_fmap,
    result_ok,
    result_err,
    result_is_ok,
    result_is_err,
    result_unwrap_or,
    result_map,
    result_and_then,
    result_bind,
    result_fmap,
};
use crate::builtins::native::regex_builtins::{
    builtin_regex_is_match,
    builtin_regex_find,
    builtin_regex_replace,
    builtin_regex_match,
};
use crate::builtins::native::file_builtins::{
    file_open_result,
    file_read_result,
    file_write_result,
    file_seek_result,
    file_close_result,
};
use crate::builtins::native::test_builtins::{
    test_assert,
    test_assert_eq,
    test_assert_not_eq,
};
use crate::builtins::native::array_builtins::{
    array_map,
    array_filter,
    array_reduce,
    array_find,
    array_some,
    array_every,
    array_flat_map,
    array_sort,
    array_sort_by,
    array_reverse,
    array_index_of,
    array_includes,
    array_concat,
    array_slice,
    array_take,
    array_drop,
    array_range,
    array_unique,
    array_flatten,
    array_zip,
    array_unzip,
    array_group_by,
    array_partition,
    array_fill,
    array_is_empty,
    array_for_each,
    array_len,
};
use crate::builtins::native::math_builtins::{
    math_abs,
    math_floor,
    math_ceil,
    math_round,
    math_min,
    math_max,
    math_pow,
    math_sin,
    math_cos,
    math_tan,
    math_sqrt,
    math_log,
    math_log10,
    math_log2,
    math_exp,
    math_asin,
    math_acos,
    math_atan,
    math_atan2,
    math_sinh,
    math_cosh,
    math_tanh,
    math_pi,
    math_e,
    math_tau,
    math_sign,
    math_clamp,
    math_random,
    math_random_int,
};
use crate::builtins::native::string_builtins::{
    string_trim,
    string_to_upper,
    string_to_lower,
    string_split,
    string_join,
    string_contains,
    string_starts_with,
    string_ends_with,
    string_index_of,
    string_slice,
    string_replace,
    string_repeat,
    string_reverse,
    string_pad_left,
    string_pad_right,
    string_chars,
    string_char_code_at,
    string_from_char_code,
    string_from_char_codes,
    string_last_index_of,
    string_replace_all,
    string_char_codes,
    string_is_empty,
    string_len,
};
use crate::builtins::native::json_builtins::{
    json_parse,
    json_stringify,
};
use crate::builtins::native::type_builtins::{
    type_int,
    type_float,
    type_str,
    type_bool,
    type_of,
    type_is_int,
    type_is_float,
    type_is_number,
    type_is_bool,
    type_is_string,
    type_is_array,
    type_is_object,
    type_is_callable,
    type_is_iterable,
    type_is_null,
    type_is_option,
    type_is_result,
};
use crate::builtins::native::object_builtins::{
    object_keys,
    object_values,
    object_entries,
    object_from_entries,
    object_has,
    object_get,
    object_set,
    object_delete,
    object_merge,
    object_is_empty,
    object_len,
};
use crate::builtins::native::time_builtins::{
    time_now,
    time_now_secs,
    time_sleep,
    time_year,
    time_month,
    time_day,
    time_hour,
    time_minute,
    time_second,
    time_day_of_week,
    time_format,
    time_to_object,
};
use crate::builtins::native::system_builtins::{
    sys_env,
    sys_set_env,
    sys_args,
    sys_exit,
    sys_cwd,
    sys_set_cwd,
    sys_exec,
    sys_platform,
    sys_arch,
};
use crate::builtins::native::http_builtins::{
    http_get,
    http_post,
    http_put,
    http_delete,
    http_patch,
    http_head,
};
use crate::builtins::native::fn_builtins::{
    fn_identity,
    fn_constant,
    fn_compose,
    fn_pipe,
    fn_apply,
    fn_call,
    fn_negate,
    fn_flip,
    fn_partial,
    fn_is_callable,
};

/// Reference-counted, interior-mutable environment handle
pub type EnvRef = Rc<RefCell<Environment>>;

/// Simple lexical environment for variables
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<EnvRef>,
    module_dir: Option<PathBuf>,
    subscriptions: HashMap<String, Vec<Object>>,
}

impl Environment {
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
            module_dir: None,
            subscriptions: HashMap::new(),
        }))
    }

    pub fn new_enclosed(outer: EnvRef) -> EnvRef {
        let module_dir = outer.borrow().module_dir.clone();
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
            module_dir,
            subscriptions: HashMap::new(),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(val) = self.store.get(name) {
            Some(val.clone())
        } else if let Some(ref outer) = self.outer {
            outer.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn snapshot(&self) -> HashMap<String, Object> {
        self.store.clone()
    }

    pub fn module_dir(&self) -> Option<PathBuf> {
        self.module_dir.clone()
    }

    pub fn set_module_dir(&mut self, dir: Option<PathBuf>) {
        self.module_dir = dir;
    }

    pub fn subscriptions(&self) -> &HashMap<String, Vec<Object>> {
        &self.subscriptions
    }

    pub fn subscriptions_mut(&mut self) -> &mut HashMap<String, Vec<Object>> {
        &mut self.subscriptions
    }
}

/// Create a new, top-level environment.
#[inline]
pub fn new_env() -> EnvRef {
    let env = Environment::new();

    {
        // Pre-bind namespaces Option, Result, Regex, File, Array, Math, String, Json and Test.
        let mut inner = env.borrow_mut();

        // Option = { Some, None, isSome, isNone, unwrapOr, map, andThen, bind, fmap }
        let mut option_methods = HashMap::new();
        option_methods.insert("Some".to_string(), Object::Builtin(option_some));
        option_methods.insert("None".to_string(), Object::Builtin(option_none));
        option_methods.insert("isSome".to_string(), Object::Builtin(option_is_some));
        option_methods.insert("isNone".to_string(), Object::Builtin(option_is_none));
        option_methods.insert("unwrapOr".to_string(), Object::Builtin(option_unwrap_or));
        option_methods.insert("map".to_string(), Object::Builtin(option_map));
        option_methods.insert("andThen".to_string(), Object::Builtin(option_and_then));
        option_methods.insert("bind".to_string(), Object::Builtin(option_bind));
        option_methods.insert("fmap".to_string(), Object::Builtin(option_fmap));
        inner.set("Option".to_string(), Object::Object(option_methods));

        // Result = { Ok, Err, isOk, isErr, unwrapOr, map, andThen, bind, fmap }
        let mut result_methods = HashMap::new();
        result_methods.insert("Ok".to_string(), Object::Builtin(result_ok));
        result_methods.insert("Err".to_string(), Object::Builtin(result_err));
        result_methods.insert("isOk".to_string(), Object::Builtin(result_is_ok));
        result_methods.insert("isErr".to_string(), Object::Builtin(result_is_err));
        result_methods.insert("unwrapOr".to_string(), Object::Builtin(result_unwrap_or));
        result_methods.insert("map".to_string(), Object::Builtin(result_map));
        result_methods.insert("andThen".to_string(), Object::Builtin(result_and_then));
        result_methods.insert("bind".to_string(), Object::Builtin(result_bind));
        result_methods.insert("fmap".to_string(), Object::Builtin(result_fmap));
        inner.set("Result".to_string(), Object::Object(result_methods));

        // Type = { int, float, str, bool, of, isInt, isFloat, isNumber, isBool, isString, isArray, isObject, isCallable, isIterable, isNull, isOption, isResult }
        let mut type_methods = HashMap::new();
        type_methods.insert("int".to_string(), Object::Builtin(type_int));
        type_methods.insert("float".to_string(), Object::Builtin(type_float));
        type_methods.insert("str".to_string(), Object::Builtin(type_str));
        type_methods.insert("bool".to_string(), Object::Builtin(type_bool));
        type_methods.insert("of".to_string(), Object::Builtin(type_of));
        type_methods.insert("isInt".to_string(), Object::Builtin(type_is_int));
        type_methods.insert("isFloat".to_string(), Object::Builtin(type_is_float));
        type_methods.insert("isNumber".to_string(), Object::Builtin(type_is_number));
        type_methods.insert("isBool".to_string(), Object::Builtin(type_is_bool));
        type_methods.insert("isString".to_string(), Object::Builtin(type_is_string));
        type_methods.insert("isArray".to_string(), Object::Builtin(type_is_array));
        type_methods.insert("isObject".to_string(), Object::Builtin(type_is_object));
        type_methods.insert("isCallable".to_string(), Object::Builtin(type_is_callable));
        type_methods.insert("isIterable".to_string(), Object::Builtin(type_is_iterable));
        type_methods.insert("isNull".to_string(), Object::Builtin(type_is_null));
        type_methods.insert("isOption".to_string(), Object::Builtin(type_is_option));
        type_methods.insert("isResult".to_string(), Object::Builtin(type_is_result));
        inner.set("Type".to_string(), Object::Object(type_methods));

        // Regex = { isMatch, find, replace, match }
        let mut regex_methods = HashMap::new();
        regex_methods.insert("isMatch".to_string(), Object::Builtin(builtin_regex_is_match));
        regex_methods.insert("find".to_string(), Object::Builtin(builtin_regex_find));
        regex_methods.insert("replace".to_string(), Object::Builtin(builtin_regex_replace));
        regex_methods.insert("match".to_string(), Object::Builtin(builtin_regex_match));
        inner.set("Regex".to_string(), Object::Object(regex_methods));

        // File = { open, read, write, seek, close } – Result-based wrappers
        let mut file_methods = HashMap::new();
        file_methods.insert("open".to_string(), Object::Builtin(file_open_result));
        file_methods.insert("read".to_string(), Object::Builtin(file_read_result));
        file_methods.insert("write".to_string(), Object::Builtin(file_write_result));
        file_methods.insert("seek".to_string(), Object::Builtin(file_seek_result));
        file_methods.insert("close".to_string(), Object::Builtin(file_close_result));
        inner.set("File".to_string(), Object::Object(file_methods));

        // Array = { map, filter, reduce, find, some, every, flatMap, sort, sortBy, reverse, indexOf, includes, concat, slice, take, drop, range, unique, flatten, zip, unzip, groupBy, partition, fill, isEmpty, forEach, len }
        let mut array_methods = HashMap::new();
        array_methods.insert("map".to_string(), Object::Builtin(array_map));
        array_methods.insert("filter".to_string(), Object::Builtin(array_filter));
        array_methods.insert("reduce".to_string(), Object::Builtin(array_reduce));
        array_methods.insert("find".to_string(), Object::Builtin(array_find));
        array_methods.insert("some".to_string(), Object::Builtin(array_some));
        array_methods.insert("every".to_string(), Object::Builtin(array_every));
        array_methods.insert("flatMap".to_string(), Object::Builtin(array_flat_map));
        array_methods.insert("sort".to_string(), Object::Builtin(array_sort));
        array_methods.insert("sortBy".to_string(), Object::Builtin(array_sort_by));
        array_methods.insert("reverse".to_string(), Object::Builtin(array_reverse));
        array_methods.insert("indexOf".to_string(), Object::Builtin(array_index_of));
        array_methods.insert("includes".to_string(), Object::Builtin(array_includes));
        array_methods.insert("concat".to_string(), Object::Builtin(array_concat));
        array_methods.insert("slice".to_string(), Object::Builtin(array_slice));
        array_methods.insert("take".to_string(), Object::Builtin(array_take));
        array_methods.insert("drop".to_string(), Object::Builtin(array_drop));
        array_methods.insert("range".to_string(), Object::Builtin(array_range));
        array_methods.insert("unique".to_string(), Object::Builtin(array_unique));
        array_methods.insert("flatten".to_string(), Object::Builtin(array_flatten));
        array_methods.insert("zip".to_string(), Object::Builtin(array_zip));
        array_methods.insert("unzip".to_string(), Object::Builtin(array_unzip));
        array_methods.insert("groupBy".to_string(), Object::Builtin(array_group_by));
        array_methods.insert("partition".to_string(), Object::Builtin(array_partition));
        array_methods.insert("fill".to_string(), Object::Builtin(array_fill));
        array_methods.insert("isEmpty".to_string(), Object::Builtin(array_is_empty));
        array_methods.insert("forEach".to_string(), Object::Builtin(array_for_each));
        array_methods.insert("len".to_string(), Object::Builtin(array_len));
        inner.set("Array".to_string(), Object::Object(array_methods));

        // Math = { abs, floor, ceil, round, min, max, pow, sin, cos, tan, sqrt, log, log10, log2, exp, asin, acos, atan, atan2, sinh, cosh, tanh, PI, E, TAU, sign, clamp, random, randomInt }
        let mut math_methods = HashMap::new();
        math_methods.insert("abs".to_string(), Object::Builtin(math_abs));
        math_methods.insert("floor".to_string(), Object::Builtin(math_floor));
        math_methods.insert("ceil".to_string(), Object::Builtin(math_ceil));
        math_methods.insert("round".to_string(), Object::Builtin(math_round));
        math_methods.insert("min".to_string(), Object::Builtin(math_min));
        math_methods.insert("max".to_string(), Object::Builtin(math_max));
        math_methods.insert("pow".to_string(), Object::Builtin(math_pow));
        math_methods.insert("sin".to_string(), Object::Builtin(math_sin));
        math_methods.insert("cos".to_string(), Object::Builtin(math_cos));
        math_methods.insert("tan".to_string(), Object::Builtin(math_tan));
        math_methods.insert("sqrt".to_string(), Object::Builtin(math_sqrt));
        math_methods.insert("log".to_string(), Object::Builtin(math_log));
        math_methods.insert("log10".to_string(), Object::Builtin(math_log10));
        math_methods.insert("log2".to_string(), Object::Builtin(math_log2));
        math_methods.insert("exp".to_string(), Object::Builtin(math_exp));
        math_methods.insert("asin".to_string(), Object::Builtin(math_asin));
        math_methods.insert("acos".to_string(), Object::Builtin(math_acos));
        math_methods.insert("atan".to_string(), Object::Builtin(math_atan));
        math_methods.insert("atan2".to_string(), Object::Builtin(math_atan2));
        math_methods.insert("sinh".to_string(), Object::Builtin(math_sinh));
        math_methods.insert("cosh".to_string(), Object::Builtin(math_cosh));
        math_methods.insert("tanh".to_string(), Object::Builtin(math_tanh));
        math_methods.insert("PI".to_string(), Object::Builtin(math_pi));
        math_methods.insert("E".to_string(), Object::Builtin(math_e));
        math_methods.insert("TAU".to_string(), Object::Builtin(math_tau));
        math_methods.insert("sign".to_string(), Object::Builtin(math_sign));
        math_methods.insert("clamp".to_string(), Object::Builtin(math_clamp));
        math_methods.insert("random".to_string(), Object::Builtin(math_random));
        math_methods.insert("randomInt".to_string(), Object::Builtin(math_random_int));
        inner.set("Math".to_string(), Object::Object(math_methods));

        // String = { trim, toUpper, toLower, split, join, contains, startsWith, endsWith, indexOf, slice, replace, repeat, reverse, padLeft, padRight, chars, charCodeAt, fromCharCode, fromCharCodes, lastIndexOf, replaceAll, charCodes, isEmpty, len }
        let mut string_methods = HashMap::new();
        string_methods.insert("trim".to_string(), Object::Builtin(string_trim));
        string_methods.insert("toUpper".to_string(), Object::Builtin(string_to_upper));
        string_methods.insert("toLower".to_string(), Object::Builtin(string_to_lower));
        string_methods.insert("split".to_string(), Object::Builtin(string_split));
        string_methods.insert("join".to_string(), Object::Builtin(string_join));
        string_methods.insert("contains".to_string(), Object::Builtin(string_contains));
        string_methods.insert("startsWith".to_string(), Object::Builtin(string_starts_with));
        string_methods.insert("endsWith".to_string(), Object::Builtin(string_ends_with));
        string_methods.insert("indexOf".to_string(), Object::Builtin(string_index_of));
        string_methods.insert("slice".to_string(), Object::Builtin(string_slice));
        string_methods.insert("replace".to_string(), Object::Builtin(string_replace));
        string_methods.insert("repeat".to_string(), Object::Builtin(string_repeat));
        string_methods.insert("reverse".to_string(), Object::Builtin(string_reverse));
        string_methods.insert("padLeft".to_string(), Object::Builtin(string_pad_left));
        string_methods.insert("padRight".to_string(), Object::Builtin(string_pad_right));
        string_methods.insert("chars".to_string(), Object::Builtin(string_chars));
        string_methods.insert("charCodeAt".to_string(), Object::Builtin(string_char_code_at));
        string_methods.insert("fromCharCode".to_string(), Object::Builtin(string_from_char_code));
        string_methods.insert("fromCharCodes".to_string(), Object::Builtin(string_from_char_codes));
        string_methods.insert("lastIndexOf".to_string(), Object::Builtin(string_last_index_of));
        string_methods.insert("replaceAll".to_string(), Object::Builtin(string_replace_all));
        string_methods.insert("charCodes".to_string(), Object::Builtin(string_char_codes));
        string_methods.insert("isEmpty".to_string(), Object::Builtin(string_is_empty));
        string_methods.insert("len".to_string(), Object::Builtin(string_len));
        inner.set("String".to_string(), Object::Object(string_methods));

        // Json = { parse, stringify }
        let mut json_methods = HashMap::new();
        json_methods.insert("parse".to_string(), Object::Builtin(json_parse));
        json_methods.insert("stringify".to_string(), Object::Builtin(json_stringify));
        inner.set("Json".to_string(), Object::Object(json_methods));

        // Test = { assert, assertEq, assertNotEq }
        let mut test_methods = HashMap::new();
        test_methods.insert("assert".to_string(), Object::Builtin(test_assert));
        test_methods.insert("assertEq".to_string(), Object::Builtin(test_assert_eq));
        test_methods.insert("assertNotEq".to_string(), Object::Builtin(test_assert_not_eq));
        inner.set("Test".to_string(), Object::Object(test_methods));

        // Object = { keys, values, entries, fromEntries, has, get, set, delete, merge, isEmpty, len }
        let mut obj_methods = HashMap::new();
        obj_methods.insert("keys".to_string(), Object::Builtin(object_keys));
        obj_methods.insert("values".to_string(), Object::Builtin(object_values));
        obj_methods.insert("entries".to_string(), Object::Builtin(object_entries));
        obj_methods.insert("fromEntries".to_string(), Object::Builtin(object_from_entries));
        obj_methods.insert("has".to_string(), Object::Builtin(object_has));
        obj_methods.insert("get".to_string(), Object::Builtin(object_get));
        obj_methods.insert("set".to_string(), Object::Builtin(object_set));
        obj_methods.insert("delete".to_string(), Object::Builtin(object_delete));
        obj_methods.insert("merge".to_string(), Object::Builtin(object_merge));
        obj_methods.insert("isEmpty".to_string(), Object::Builtin(object_is_empty));
        obj_methods.insert("len".to_string(), Object::Builtin(object_len));
        inner.set("Obj".to_string(), Object::Object(obj_methods));

        // Time = { now, nowSecs, sleep, year, month, day, hour, minute, second, dayOfWeek, format, toObject }
        let mut time_methods = HashMap::new();
        time_methods.insert("now".to_string(), Object::Builtin(time_now));
        time_methods.insert("nowSecs".to_string(), Object::Builtin(time_now_secs));
        time_methods.insert("sleep".to_string(), Object::Builtin(time_sleep));
        time_methods.insert("year".to_string(), Object::Builtin(time_year));
        time_methods.insert("month".to_string(), Object::Builtin(time_month));
        time_methods.insert("day".to_string(), Object::Builtin(time_day));
        time_methods.insert("hour".to_string(), Object::Builtin(time_hour));
        time_methods.insert("minute".to_string(), Object::Builtin(time_minute));
        time_methods.insert("second".to_string(), Object::Builtin(time_second));
        time_methods.insert("dayOfWeek".to_string(), Object::Builtin(time_day_of_week));
        time_methods.insert("format".to_string(), Object::Builtin(time_format));
        time_methods.insert("toObject".to_string(), Object::Builtin(time_to_object));
        inner.set("Time".to_string(), Object::Object(time_methods));

        // Sys = { env, setEnv, args, exit, cwd, setCwd, exec, platform, arch }
        let mut sys_methods = HashMap::new();
        sys_methods.insert("env".to_string(), Object::Builtin(sys_env));
        sys_methods.insert("setEnv".to_string(), Object::Builtin(sys_set_env));
        sys_methods.insert("args".to_string(), Object::Builtin(sys_args));
        sys_methods.insert("exit".to_string(), Object::Builtin(sys_exit));
        sys_methods.insert("cwd".to_string(), Object::Builtin(sys_cwd));
        sys_methods.insert("setCwd".to_string(), Object::Builtin(sys_set_cwd));
        sys_methods.insert("exec".to_string(), Object::Builtin(sys_exec));
        sys_methods.insert("platform".to_string(), Object::Builtin(sys_platform));
        sys_methods.insert("arch".to_string(), Object::Builtin(sys_arch));
        inner.set("Sys".to_string(), Object::Object(sys_methods));

        // HTTP = { get, post, put, delete, patch, head }
        let mut http_methods = HashMap::new();
        http_methods.insert("get".to_string(), Object::Builtin(http_get));
        http_methods.insert("post".to_string(), Object::Builtin(http_post));
        http_methods.insert("put".to_string(), Object::Builtin(http_put));
        http_methods.insert("delete".to_string(), Object::Builtin(http_delete));
        http_methods.insert("patch".to_string(), Object::Builtin(http_patch));
        http_methods.insert("head".to_string(), Object::Builtin(http_head));
        inner.set("HTTP".to_string(), Object::Object(http_methods));

        // Fn = { identity, constant, compose, pipe, apply, call, negate, flip, partial, isCallable }
        let mut fn_methods = HashMap::new();
        fn_methods.insert("identity".to_string(), Object::Builtin(fn_identity));
        fn_methods.insert("constant".to_string(), Object::Builtin(fn_constant));
        fn_methods.insert("compose".to_string(), Object::Builtin(fn_compose));
        fn_methods.insert("pipe".to_string(), Object::Builtin(fn_pipe));
        fn_methods.insert("apply".to_string(), Object::Builtin(fn_apply));
        fn_methods.insert("call".to_string(), Object::Builtin(fn_call));
        fn_methods.insert("negate".to_string(), Object::Builtin(fn_negate));
        fn_methods.insert("flip".to_string(), Object::Builtin(fn_flip));
        fn_methods.insert("partial".to_string(), Object::Builtin(fn_partial));
        fn_methods.insert("isCallable".to_string(), Object::Builtin(fn_is_callable));
        inner.set("Fn".to_string(), Object::Object(fn_methods));
    }

    env
}

/// Create a new environment enclosed within an existing outer environment.
#[inline]
pub fn new_enclosed_env(outer: EnvRef) -> EnvRef {
    Environment::new_enclosed(outer)
}

fn root_env(env: EnvRef) -> EnvRef {
    let mut current = Rc::clone(&env);
    loop {
        let next = {
            let borrow = current.borrow();
            borrow.outer.clone()
        };

        match next {
            Some(next_env) => current = next_env,
            None => break,
        }
    }
    current
}

pub fn register_subscription(tag: &str, func: Object, env: EnvRef) {
    let root = root_env(env);
    root.borrow_mut()
        .subscriptions_mut()
        .entry(tag.to_string())
        .or_default()
        .push(func);
}

pub fn subscribers_for_tag(tag: &str, env: EnvRef) -> Vec<Object> {
    let root = root_env(env);
    root.borrow()
        .subscriptions()
        .get(tag)
        .cloned()
        .unwrap_or_default()
}


