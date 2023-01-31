use std::{fmt::Debug, str};

use nom::{
    bytes::complete::tag,
    character::complete::not_line_ending,
    combinator::{map, map_res},
    IResult,
};

use super::{parsers::*, RadioModule};
use crate::spectrum_analyzer::Model;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupInfo<M: Debug + Clone + Copy + TryFrom<u8> + PartialEq + Eq = Model> {
    pub main_radio_module: RadioModule<M>,
    pub expansion_radio_module: Option<RadioModule<M>>,
    pub firmware_version: String,
}

impl<M: Debug + Copy + TryFrom<u8> + Eq + PartialEq> SetupInfo<M> {
    pub(crate) fn parse_with_prefix<'a>(
        bytes: &'a [u8],
        prefix: &'static [u8],
    ) -> IResult<&'a [u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(prefix)(bytes)?;

        // Parse the main model
        let (bytes, main_model) = map_res(parse_num(3), M::try_from)(bytes)?;

        let (bytes, _) = tag(",")(bytes)?;

        // Parse the expansion model
        let (bytes, exp_model) = map_res(parse_num(3), |num| {
            if let Ok(model) = M::try_from(num) {
                Ok(Some(model))
            } else if num == 255 {
                Ok(None)
            } else {
                Err(())
            }
        })(bytes)?;

        let (bytes, _) = tag(",")(bytes)?;

        // Parse the firmware version
        let (bytes, firmware_version) =
            map(map_res(not_line_ending, str::from_utf8), str::to_string)(bytes)?;

        // Consume \r or \r\n line ending and make sure there aren't any bytes left
        let (bytes, _) = parse_opt_line_ending(bytes)?;

        Ok((
            bytes,
            SetupInfo {
                main_radio_module: RadioModule::Main { model: main_model },
                expansion_radio_module: exp_model.map(|model| RadioModule::Expansion { model }),
                firmware_version,
            },
        ))
    }
}
