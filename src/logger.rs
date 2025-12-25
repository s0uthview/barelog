use crate::level::Level;
use core::fmt::Arguments;
use spin::Mutex;

#[cfg(feature = "alloc")]
use crate::subscriber::get_subscribers;

#[cfg(not(feature = "alloc"))]
use crate::subscriber::get_subscriber;

/// The maximum log level for filtering output.
///
/// Messages below this level will be ignored.
static MAX_LEVEL: Mutex<Level> = Mutex::new(Level::Info);

/// Sets the maximum log level.
///
/// Only messages at or above this level will be logged.
///
/// # Arguments
/// * `level` - The most verbose level to allow.
pub fn set_max_level(level: Level) {
    *MAX_LEVEL.lock() = level;
}

/// Gets the current maximum log level.
pub fn get_max_level() -> Level {
    *MAX_LEVEL.lock()
}

/// Logs a message at the given level.
///
/// This is the core logging function. End users should prefer the macros (`info!`, `warn!`, etc).
///
/// # Arguments
/// * `level` - The severity of the message.
/// * `args` - The formatted message arguments.
pub fn log(level: Level, args: Arguments) {
    if level >= get_max_level() {
        #[cfg(not(feature = "alloc"))]
        if let Some(sub) = get_subscriber() {
            sub.log(level, args);
        }

        #[cfg(feature = "alloc")]
        for sub in get_subscribers().iter() {
            sub.log(level, args);
        }
    }
}

/// Logs a message at a specific level with formatting.
///
/// Prefer using the level-specific macros (`info!`, `warn!`, etc) for convenience.
///
/// # Example
/// ```rust
/// barelog::log!(barelog::Level::Info, "Hello {}", "world");
/// ```
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        $crate::logger::log(
            $level,
            core::format_args!(
                "[{}::{}] [{}] {}",
                core::module_path!(),
                core::line!(),
                $level.as_str(),
                core::format_args!($($arg)*)
            )
        );
    }};
}

/// Logs a message at the TRACE level.
///
/// Usage: `trace!("message: {}", value)`
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::log!(
            $crate::Level::Trace,
            $($arg)*
        );
    }};
}

/// Logs a message at the DEBUG level.
///
/// Usage: `debug!("message: {}", value)`
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::log!(
            $crate::Level::Debug,
            $($arg)*
        );
    }};
}

/// Logs a message at the INFO level.
///
/// Usage: `info!("message: {}", value)`
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log!(
            $crate::Level::Info,
            $($arg)*
        );
    }};
}

/// Logs a message at the WARN level.
///
/// Usage: `warn!("message: {}", value)`
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log!(
            $crate::Level::Warn,
            $($arg)*
        );
    }};
}

/// Logs a message at the ERROR level.
///
/// Usage: `error!("message: {}", value)`
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log!(
            $crate::Level::Error,
            $($arg)*
        );
    }};
}

/// Logs a message at the FATAL level.
///
/// Usage: `fatal!("message: {}", value)`
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        $crate::log!(
            $crate::Level::Fatal,
            $($arg)*
        );
    }};
}
