use std::{
    fmt::{write, Display},
    str,
};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::is_alphanumeric,
    combinator::{map, map_res},
    sequence::preceded,
    IResult,
};

use super::parsers::*;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SerialNumber {
    serial_number: String,
}

impl SerialNumber {
    pub const PREFIX: &'static [u8] = b"#Sn";

    pub fn as_str(&self) -> &str {
        &self.serial_number
    }

    pub(crate) fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (bytes, serial_number) = preceded(
            tag(SerialNumber::PREFIX),
            map(
                map_res(take_while_m_n(16, 16, is_alphanumeric), str::from_utf8),
                str::to_string,
            ),
        )(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let (bytes, _) = parse_opt_line_ending(bytes)?;

        Ok((bytes, SerialNumber { serial_number }))
    }
}

impl AsRef<str> for SerialNumber {
    fn as_ref(&self) -> &str {
        &self.serial_number
    }
}

impl Display for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_with_invalid_prefix() {
        assert!(SerialNumber::parse(b"$Sn0SME38SI2X7NGR48".as_ref()).is_err());
    }

    #[test]
    fn accept_valid_serial_number() {
        assert!(SerialNumber::parse(b"#Sn0SME38SI2X7NGR48".as_ref()).is_ok());
        assert!(SerialNumber::parse(b"#SnB3AK7AL7CACAA74M\r\n".as_ref()).is_ok());
    }
}
