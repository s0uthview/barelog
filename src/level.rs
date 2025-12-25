/// Represents the severity of a log message.
///
/// Levels are ordered from least to most severe:
/// - `Trace`: Very low-level, verbose output for debugging.
/// - `Debug`: Debugging information.
/// - `Info`: General information about normal operation.
/// - `Warn`: Something unexpected or suboptimal happened, but the program can continue.
/// - `Error`: An error occurred, but the program can still run.
/// - `Fatal`: A critical error after which the program cannot continue.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Very low-level, verbose output for debugging.
    Trace,
    /// Debugging information.
    Debug,
    /// General information about normal operation.
    Info,
    /// Something unexpected or suboptimal happened, but the program can continue.
    Warn,
    /// An error occurred, but the program can still run.
    Error,
    /// A critical error after which the program cannot continue.
    Fatal,
}

impl Level {
    /// Returns the string representation of the log level (e.g., "INFO").
    pub const fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }

    /// Returns the ANSI color code for this level (if the `color` feature is enabled).
    #[cfg(feature = "color")]
    pub const fn color_code(&self) -> &'static str {
        match self {
            Level::Trace => "\x1B[37m", // white
            Level::Debug => "\x1B[36m", // cyan
            Level::Info => "\x1B[32m",  // green
            Level::Warn => "\x1B[33m",  // yellow
            Level::Error => "\x1B[31m", // red
            Level::Fatal => "\x1B[35m", // magenta
        }
    }

    /// Returns the ANSI reset code (if the `color` feature is enabled).
    #[cfg(feature = "color")]
    pub const fn reset_code(&self) -> &'static str {
        "\x1B[0m"
    }
}
