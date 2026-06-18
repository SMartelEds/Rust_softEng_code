#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("[ERROR] {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        println!("[DEBUG] {}", format_args!($($arg)*));
    };
}
