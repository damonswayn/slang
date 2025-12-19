use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

use crate::env::EnvRef;
use crate::object::Object;

fn expect_one_arg(mut args: Vec<Object>, name: &str) -> Result<Object, Object> {
    if args.len() != 1 {
        return Err(Object::error(format!("{name} expects exactly 1 argument")));
    }
    Ok(args.pop().unwrap())
}

/// Time::now() -> integer (Unix timestamp in milliseconds)
pub(crate) fn time_now(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Time::now expects no arguments");
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO);

    Object::Integer(now.as_millis() as i64)
}

/// Time::nowSecs() -> integer (Unix timestamp in seconds)
pub(crate) fn time_now_secs(args: Vec<Object>, _env: EnvRef) -> Object {
    if !args.is_empty() {
        return Object::error("Time::nowSecs expects no arguments");
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO);

    Object::Integer(now.as_secs() as i64)
}

/// Time::sleep(ms) -> null (pauses execution for ms milliseconds)
pub(crate) fn time_sleep(args: Vec<Object>, _env: EnvRef) -> Object {
    let ms = match expect_one_arg(args, "Time::sleep") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ms_val = match ms {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::sleep expects integer milliseconds, got {:?}",
                other
            ))
        }
    };

    if ms_val < 0 {
        return Object::error("Time::sleep milliseconds must be non-negative");
    }

    thread::sleep(Duration::from_millis(ms_val as u64));
    Object::Null
}

// Helper to get components from a Unix timestamp in milliseconds
fn timestamp_to_components(ts_ms: i64) -> (i32, u32, u32, u32, u32, u32, u32) {
    // Convert to seconds
    let ts_secs = ts_ms / 1000;
    
    // Days since Unix epoch
    let days = (ts_secs / 86400) as i32;
    let time_of_day = (ts_secs % 86400) as u32;
    
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;
    let millis = (ts_ms % 1000) as u32;
    
    // Calculate year, month, day from days since epoch
    // This is a simplified calculation
    let mut year = 1970i32;
    let mut remaining_days = days;
    
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    // Handle negative timestamps (before 1970)
    if days < 0 {
        year = 1970;
        remaining_days = 0;
    }
    
    let days_in_months: [u32; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    
    let mut month = 1u32;
    let mut day = remaining_days as u32 + 1;
    
    for (i, &days_in_month) in days_in_months.iter().enumerate() {
        if day <= days_in_month {
            month = (i + 1) as u32;
            break;
        }
        day -= days_in_month;
    }
    
    (year, month, day, hour, minute, second, millis)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn day_of_week(ts_ms: i64) -> u32 {
    // Jan 1, 1970 was a Thursday (day 4, where Sunday = 0)
    let days = (ts_ms / 86400000) as i32;
    ((days + 4) % 7).abs() as u32
}

/// Time::year(ts) -> integer
pub(crate) fn time_year(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::year") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::year expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (year, _, _, _, _, _, _) = timestamp_to_components(ts_val);
    Object::Integer(year as i64)
}

/// Time::month(ts) -> integer (1-12)
pub(crate) fn time_month(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::month") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::month expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (_, month, _, _, _, _, _) = timestamp_to_components(ts_val);
    Object::Integer(month as i64)
}

/// Time::day(ts) -> integer (1-31)
pub(crate) fn time_day(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::day") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::day expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (_, _, day, _, _, _, _) = timestamp_to_components(ts_val);
    Object::Integer(day as i64)
}

/// Time::hour(ts) -> integer (0-23)
pub(crate) fn time_hour(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::hour") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::hour expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (_, _, _, hour, _, _, _) = timestamp_to_components(ts_val);
    Object::Integer(hour as i64)
}

/// Time::minute(ts) -> integer (0-59)
pub(crate) fn time_minute(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::minute") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::minute expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (_, _, _, _, minute, _, _) = timestamp_to_components(ts_val);
    Object::Integer(minute as i64)
}

/// Time::second(ts) -> integer (0-59)
pub(crate) fn time_second(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::second") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::second expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (_, _, _, _, _, second, _) = timestamp_to_components(ts_val);
    Object::Integer(second as i64)
}

/// Time::dayOfWeek(ts) -> integer (0=Sunday, 1=Monday, ..., 6=Saturday)
pub(crate) fn time_day_of_week(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::dayOfWeek") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::dayOfWeek expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    Object::Integer(day_of_week(ts_val) as i64)
}

/// Time::format(ts, fmt) -> string
/// Format specifiers: %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second)
pub(crate) fn time_format(mut args: Vec<Object>, _env: EnvRef) -> Object {
    if args.len() != 2 {
        return Object::error("Time::format expects exactly 2 arguments (timestamp, format)");
    }

    let fmt = args.pop().unwrap();
    let ts = args.pop().unwrap();

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::format expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let fmt_str = match fmt {
        Object::String(s) => s,
        other => {
            return Object::error(format!(
                "Time::format expects string format, got {:?}",
                other
            ))
        }
    };

    let (year, month, day, hour, minute, second, _) = timestamp_to_components(ts_val);

    let result = fmt_str
        .replace("%Y", &format!("{:04}", year))
        .replace("%m", &format!("{:02}", month))
        .replace("%d", &format!("{:02}", day))
        .replace("%H", &format!("{:02}", hour))
        .replace("%M", &format!("{:02}", minute))
        .replace("%S", &format!("{:02}", second));

    Object::String(result)
}

/// Time::toObject(ts) -> { year, month, day, hour, minute, second, dayOfWeek }
pub(crate) fn time_to_object(args: Vec<Object>, _env: EnvRef) -> Object {
    let ts = match expect_one_arg(args, "Time::toObject") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let ts_val = match ts {
        Object::Integer(i) => i,
        other => {
            return Object::error(format!(
                "Time::toObject expects integer timestamp, got {:?}",
                other
            ))
        }
    };

    let (year, month, day, hour, minute, second, _) = timestamp_to_components(ts_val);
    let dow = day_of_week(ts_val);

    let mut map = std::collections::HashMap::new();
    map.insert("year".to_string(), Object::Integer(year as i64));
    map.insert("month".to_string(), Object::Integer(month as i64));
    map.insert("day".to_string(), Object::Integer(day as i64));
    map.insert("hour".to_string(), Object::Integer(hour as i64));
    map.insert("minute".to_string(), Object::Integer(minute as i64));
    map.insert("second".to_string(), Object::Integer(second as i64));
    map.insert("dayOfWeek".to_string(), Object::Integer(dow as i64));

    Object::Object(map)
}

