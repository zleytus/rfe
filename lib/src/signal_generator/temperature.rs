use std::{convert::TryFrom, ops::RangeInclusive};

use nom::Parser;
use nom::{bytes::complete::tag, combinator::map_res, number::complete::u8 as nom_u8};
use num_enum::TryFromPrimitive;

use crate::common::MessageParseError;
use crate::rf_explorer::parsers::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Temperature {
    MinusTenToZero = b'0',
    ZeroToTen = b'1',
    TenToTwenty = b'2',
    TwentyToThirty = b'3',
    ThirtyToForty = b'4',
    FortyToFifty = b'5',
    FiftyToSixty = b'6',
}

impl Temperature {
    pub(crate) const PREFIX: &'static [u8] = b"#T:";

    pub fn range(&self) -> RangeInclusive<i8> {
        match self {
            Temperature::MinusTenToZero => -10..=0,
            Temperature::ZeroToTen => 0..=10,
            Temperature::TenToTwenty => 10..=20,
            Temperature::TwentyToThirty => 20..=30,
            Temperature::ThirtyToForty => 30..=40,
            Temperature::FortyToFifty => 40..=50,
            Temperature::FiftyToSixty => 50..=60,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Temperature {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Temperature::PREFIX)(bytes)?;

        // Parse the temperature
        let (bytes, temperature) = map_res(nom_u8, Temperature::try_from).parse(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(temperature)
    }
}
