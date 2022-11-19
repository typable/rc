#[macro_export]
macro_rules! log {
    ($level:expr, $($message:tt)*) => {{
        println!(
            "{}{}\x1b[0m",
            match $level {
                LogLevel::Info => "",
                LogLevel::Warn => "\x1b[33m",
                LogLevel::Error => "\x1b[31m",
                LogLevel::Success => "\x1b[32m",
            },
            format_args!($($message)*)
        );
    }};
}

#[macro_export]
macro_rules! info {
    ($($message:tt)*) => {{
        log!(LogLevel::Info, $($message)*);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($message:tt)*) => {{
        log!(LogLevel::Warn, $($message)*);
    }};
}

#[macro_export]
macro_rules! error {
    ($($message:tt)*) => {{
        log!(LogLevel::Error, $($message)*);
    }};
}

#[macro_export]
macro_rules! success {
    ($($message:tt)*) => {{
        log!(LogLevel::Success, $($message)*);
    }};
}

pub enum LogLevel {
    Info,
    Warn,
    Error,
    Success,
}
