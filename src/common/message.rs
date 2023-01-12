use nom::error::Error;
use thiserror::Error;

pub trait Message: Sized {
    fn parse(bytes: &[u8]) -> Result<Self, MessageParseError>;
}

#[derive(Error, Debug)]
pub enum MessageParseError {
    #[error("Attempted to parse an incomplete message")]
    Incomplete,

    #[error("Attempted to parse an invalid message")]
    Invalid,
}

impl From<nom::Err<Error<&[u8]>>> for MessageParseError {
    fn from(error: nom::Err<Error<&[u8]>>) -> Self {
        match error {
            nom::Err::Incomplete(_) => MessageParseError::Incomplete,
            _ => MessageParseError::Invalid,
        }
    }
}
