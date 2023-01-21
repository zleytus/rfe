use std::fmt::Display;

use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::common::parsers::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InputStage {
    Direct = b'0',
    Attenuator30dB = b'1',
    Lna25dB = b'2',
    Attenuator60dB = b'3',
    Lna12dB = b'4',
}

impl InputStage {
    pub const PREFIX: &'static [u8] = b"#a";

    pub(crate) fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(InputStage::PREFIX)(bytes)?;

        // Parse the input stage
        let (bytes, input_stage) =
            map_res(take(1usize), |byte: &[u8]| InputStage::try_from(byte[0]))(bytes)?;

        // Consume \r or \r\n line ending and make sure there aren't any bytes left
        let (bytes, _) = parse_opt_line_ending(bytes)?;

        Ok((bytes, input_stage))
    }
}

impl Display for InputStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input_stage = match self {
            InputStage::Direct => "Direct",
            InputStage::Attenuator30dB => "Attenuator 30dB",
            InputStage::Lna25dB => "LNA 25dB",
            InputStage::Attenuator60dB => "Attenuator 60dB",
            InputStage::Lna12dB => "Attenuator 12dB",
        };
        write!(f, "{input_stage}")
    }
}
