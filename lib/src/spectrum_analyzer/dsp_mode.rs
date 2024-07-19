use std::{convert::TryFrom, fmt::Display};

use nom::{bytes::complete::tag, combinator::map_res};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::common::MessageParseError;
use crate::rf_explorer::parsers::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum DspMode {
    #[default]
    Auto = 0,
    Filter,
    Fast,
    NoImg,
}

impl DspMode {
    pub(crate) const PREFIX: &'static [u8] = b"DSP:";
}

impl<'a> TryFrom<&'a [u8]> for DspMode {
    type Error = MessageParseError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the prefix of the message
        let (bytes, _) = tag(DspMode::PREFIX)(bytes)?;

        // Parse the DSP mode
        let (bytes, dsp_mode) = map_res(parse_num::<u8>(1u8), DspMode::try_from)(bytes)?;

        // Consume \r or \r\n line ending and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(dsp_mode)
    }
}

impl Display for DspMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dsp_mode = match self {
            Self::Auto => "Auto",
            Self::Filter => "Filter",
            Self::Fast => "Fast",
            Self::NoImg => "NoImg",
        };
        write!(f, "{dsp_mode}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_valid_dsp_mode_message() {
        let bytes = b"DSP:0";
        let dsp_mode = DspMode::try_from(bytes.as_ref()).unwrap();
        assert_eq!(dsp_mode, DspMode::Auto);
    }
}
