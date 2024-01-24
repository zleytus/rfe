use std::convert::TryFrom;

use nom::{bytes::complete::tag, combinator::map_res, number::complete::u8 as nom_u8};
use num_enum::TryFromPrimitive;

use crate::common::MessageParseError;
use crate::rf_explorer::parsers::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum TrackingStatus {
    #[default]
    Disabled = 0,
    Enabled,
}

impl TrackingStatus {
    pub(crate) const PREFIX: &'static [u8] = b"#K";
}

impl<'a> TryFrom<&'a [u8]> for TrackingStatus {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(TrackingStatus::PREFIX)(bytes)?;

        // Parse the tracking status
        let (bytes, tracking_status) = map_res(nom_u8, TrackingStatus::try_from)(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(tracking_status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_valid_tracking_status_message() {
        let bytes = [b'#', b'K', 0];
        let tracking_status = TrackingStatus::try_from(bytes.as_ref()).unwrap();
        assert_eq!(tracking_status, TrackingStatus::Disabled);
    }
}
