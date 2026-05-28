use std::fmt::Debug;

use nom::{Err, error::Error};
use thiserror::Error;

use super::ConnectionResult;

/// Storage and synchronization contract for messages read by [`Device`](crate::Device).
pub trait MessageContainer: Default + Debug + Send + Sync {
    /// Parsed message type accepted by this container.
    type Message: for<'a> TryFrom<&'a [u8], Error = MessageParseError<'a>> + Debug;

    /// Stores a parsed message and wakes any waiters interested in that message.
    fn cache_message(&self, message: Self::Message);

    /// Waits until the initial device-identification messages have been received.
    fn wait_for_device_info(&self) -> ConnectionResult<()>;
}

#[derive(Error, Debug, Eq, PartialEq)]
/// Error returned when parsing a device message fails.
pub enum MessageParseError<'a> {
    /// More bytes are needed to parse a complete message.
    #[error("Attempted to parse an incomplete message")]
    Incomplete,

    /// The message was interrupted by another message.
    #[error("Attempted to parse a truncated message")]
    Truncated {
        /// Bytes following the truncated message, if any.
        remainder: Option<&'a [u8]>,
    },

    /// The message bytes do not match the expected format.
    #[error("Attempted to parse an invalid message")]
    Invalid,

    /// The message prefix does not identify a known message type.
    #[error("Attempted to parse an unknown message type")]
    UnknownMessageType,
}

impl<'a> From<Err<Error<&[u8]>>> for MessageParseError<'a> {
    fn from(error: Err<Error<&[u8]>>) -> Self {
        match error {
            Err::Incomplete(_) => MessageParseError::Incomplete,
            _ => MessageParseError::Invalid,
        }
    }
}
