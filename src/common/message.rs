use nom::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageParseError {
    #[error("Attempted to parse an incomplete message")]
    Incomplete,

    #[error("Attempted to parse an invalid message")]
    Invalid,

    #[error("Attempted to parse an unknown message type")]
    UnknownMessageType,
}

impl From<nom::Err<Error<&[u8]>>> for MessageParseError {
    fn from(error: nom::Err<Error<&[u8]>>) -> Self {
        match error {
            nom::Err::Incomplete(_) => MessageParseError::Incomplete,
            _ => MessageParseError::Invalid,
        }
    }
}
