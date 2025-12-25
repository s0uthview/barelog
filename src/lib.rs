//! # barelog
//!
//! A minimal, `no_std`-friendly logging library for close-to-metal Rust environments.
//!
//! - Supports multiple log levels (trace, debug, info, warn, error, fatal)
//! - Pluggable subscriber system for log output
//! - Thread-safe, simple API
//! - Optional `alloc` support for multiple subscribers
//!
//! ```
#![no_std]

pub mod level;
pub mod logger;
pub mod subscriber;

pub use level::Level;
pub use logger::{get_max_level, log, set_max_level};

#[cfg(not(feature = "alloc"))]
pub use subscriber::{Subscriber, set_subscriber};

#[cfg(feature = "alloc")]
pub use subscriber::{Subscriber, add_subscriber};
