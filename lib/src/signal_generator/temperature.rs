use std::{convert::TryFrom, ops::RangeInclusive};

use nom::Parser;
use nom::{bytes::complete::tag, combinator::map_res, number::complete::u8 as nom_u8};
use num_enum::TryFromPrimitive;

use crate::common::MessageParseError;
use crate::rf_explorer::parsers::*;

/// Temperature range reported by the signal generator.
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Temperature {
    /// Temperature is between -10 C and 0 C.
    MinusTenToZero = b'0',
    /// Temperature is between 0 C and 10 C.
    ZeroToTen = b'1',
    /// Temperature is between 10 C and 20 C.
    TenToTwenty = b'2',
    /// Temperature is between 20 C and 30 C.
    TwentyToThirty = b'3',
    /// Temperature is between 30 C and 40 C.
    ThirtyToForty = b'4',
    /// Temperature is between 40 C and 50 C.
    FortyToFifty = b'5',
    /// Temperature is between 50 C and 60 C.
    FiftyToSixty = b'6',
}

impl Temperature {
    pub(crate) const PREFIX: &'static [u8] = b"#T:";

    /// Returns the inclusive temperature range in degrees Celsius.
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
