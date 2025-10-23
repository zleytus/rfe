use std::{fmt::Debug, str};

use nom::{
    bytes::complete::tag,
    character::complete::not_line_ending,
    combinator::{map, map_res},
    Parser,
};

use super::parsers::*;
use crate::common::MessageParseError;
use crate::spectrum_analyzer::Model;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SetupInfo<
    M: Debug + Clone + Copy + TryFrom<u8> + PartialEq + Eq + Default = Model,
> {
    pub main_radio_model: Option<M>,
    pub expansion_radio_model: Option<M>,
    pub firmware_version: String,
}

impl<M: Debug + Copy + TryFrom<u8> + Eq + PartialEq + Default> SetupInfo<M> {
    pub(crate) fn try_from_with_prefix<'a>(
        bytes: &'a [u8],
        prefix: &'static [u8],
    ) -> Result<Self, MessageParseError<'a>> {
        // Parse the prefix of the message
        let (bytes, _) = tag(prefix)(bytes)?;

        // Parse the main radio's model
        let (bytes, main_radio_model) = map_res(parse_num(3), |num| {
            if let Ok(model) = M::try_from(num) {
                Ok(Some(model))
            } else if num == 255 {
                Ok(None)
            } else {
                Err(())
            }
        })
        .parse(bytes)?;

        let (bytes, _) = tag(",")(bytes)?;

        // Parse the expansion radio's model
        let (bytes, expansion_radio_model) = map_res(parse_num(3), |num| {
            if let Ok(model) = M::try_from(num) {
                Ok(Some(model))
            } else if num == 255 {
                Ok(None)
            } else {
                Err(())
            }
        })
        .parse(bytes)?;

        let (bytes, _) = tag(",")(bytes)?;

        // Parse the firmware version
        let (bytes, firmware_version) =
            map(map_res(not_line_ending, str::from_utf8), str::to_string).parse(bytes)?;

        // Consume \r or \r\n line ending and make sure there aren't any bytes left
        let _ = parse_opt_line_ending(bytes)?;

        Ok(SetupInfo {
            main_radio_model,
            expansion_radio_model,
            firmware_version,
        })
    }
}
