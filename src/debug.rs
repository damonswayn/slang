use std::sync::atomic::{AtomicBool, Ordering};

pub static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

pub fn enable_debug_mode() {
    DEBUG_MODE.store(true, Ordering::SeqCst);
}

pub fn disable_debug_mode() {
    DEBUG_MODE.store(false, Ordering::SeqCst);
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if crate::debug::DEBUG_MODE.load(std::sync::atomic::Ordering::Relaxed) {
            println!($($arg)*);
        }
    };
}