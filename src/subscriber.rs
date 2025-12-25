use crate::level::Level;
use core::fmt::Arguments;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Trait for log event consumers.
///
/// Implement this trait to receive log messages from the logger.
pub trait Subscriber: Sync {
    /// Called for each log event.
    ///
    /// # Arguments
    /// * `level` - The severity of the message.
    /// * `args` - The formatted message arguments.
    fn log(&self, level: Level, args: Arguments);
}

#[cfg(not(feature = "alloc"))]
use spin::Once;

/// Global static for the single subscriber.
///
/// Only available when the `alloc` feature is **not** enabled.
/// When `alloc` is disabled, only one subscriber can be registered.
#[cfg(not(feature = "alloc"))]
static LOGGER: Once<&'static dyn Subscriber> = Once::new();

#[cfg(feature = "alloc")]
use spin::Mutex;

/// Global static for multiple subscribers.
///
/// Only available when the `alloc` feature **is** enabled.
/// When `alloc` is enabled, multiple subscribers can be registered.
#[cfg(feature = "alloc")]
static SUBSCRIBERS: Mutex<Vec<&'static dyn Subscriber>> = Mutex::new(Vec::new());

/// Registers a single subscriber.
///
/// Only available when the `alloc` feature is **not** enabled.
/// When `alloc` is disabled, only one subscriber can be registered.
///
/// # Arguments
/// * `subscriber` - The log consumer to receive all log events.
#[cfg(not(feature = "alloc"))]
pub fn set_subscriber(subscriber: &'static dyn Subscriber) {
    LOGGER.call_once(|| subscriber);
}

/// Adds a subscriber.
///
/// Only available when the `alloc` feature **is** enabled.
/// When `alloc` is enabled, multiple subscribers can be registered.
///
/// # Arguments
/// * `subscriber` - The log consumer to add.
#[cfg(feature = "alloc")]
pub fn add_subscriber(subscriber: &'static dyn Subscriber) {
    let mut subs = SUBSCRIBERS.lock();
    subs.push(subscriber);
}

/// Returns the registered subscriber.
///
/// Only available when the `alloc` feature is **not** enabled.
#[cfg(not(feature = "alloc"))]
pub(crate) fn get_subscriber() -> Option<&'static dyn Subscriber> {
    LOGGER.get().copied()
}

/// Returns the registered subscribers.
///
/// Only available when the `alloc` feature **is** enabled.
#[cfg(feature = "alloc")]
pub(crate) fn get_subscribers() -> Vec<&'static dyn Subscriber> {
    SUBSCRIBERS.lock().clone()
}
