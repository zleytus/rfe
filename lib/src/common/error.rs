use std::{io, time::Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("This operation requires firmware version {} or later", .0)]
    IncompatibleFirmware(String),

    #[error("Invalid input: {}", .0)]
    InvalidInput(String),

    #[error("Invalid operation: {}", .0)]
    InvalidOperation(String),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Failed to complete the operation within the timeout duration ({} ms)", .0.as_millis())]
    TimedOut(Duration),
}

pub type Result<T> = std::result::Result<T, Error>;
