//! Error types for barelog.
//!

/// Errors that can occur in the logger.
#[derive(Debug, PartialEq, Eq)]
pub enum BarelogError {
    /// Attempted to register a subscriber when one is already set (no-alloc mode).
    #[cfg(not(feature = "alloc"))]
    SubscriberAlreadySet,

    /// Attempted to add a subscriber when the list is full (alloc mode, if a limit is set).
    #[cfg(feature = "alloc")]
    SubscriberListFull,

    /// Other errors.
    Other(&'static str),
}
