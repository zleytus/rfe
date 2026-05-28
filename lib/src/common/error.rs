use std::{io, time::Duration};

use thiserror::Error;

#[derive(Error, Debug)]
/// Error returned by high-level RF Explorer operations.
pub enum Error {
    /// The connected device firmware is older than the operation requires.
    #[error("This operation requires firmware version {} or later", .0)]
    IncompatibleFirmware(String),

    /// A caller supplied an invalid value.
    #[error("Invalid input: {}", .0)]
    InvalidInput(String),

    /// The requested operation is not valid in the device's current state.
    #[error("Invalid operation: {}", .0)]
    InvalidOperation(String),

    /// An underlying I/O operation failed.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// The device did not respond before the timeout elapsed.
    #[error("Failed to complete the operation within the timeout duration ({} ms)", .0.as_millis())]
    TimedOut(Duration),
}

/// Result type returned by high-level RF Explorer operations.
pub type Result<T> = std::result::Result<T, Error>;
