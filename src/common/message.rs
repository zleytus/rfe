use nom::error::Error;

pub trait Message: Sized {
    fn parse(bytes: &[u8]) -> Result<Self, nom::Err<Error<&[u8]>>>;
}
