#[macro_export]
macro_rules! step_log {
    ($($arg:tt)*) => {
        println!("[STEP] {}", format_args!($($arg)*));
    };
}
