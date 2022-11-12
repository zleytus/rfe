use super::Device;
use crate::{Model, SpectrumAnalyzer};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{line_ending, not_line_ending},
    combinator::{all_consuming, map, map_res, opt},
    IResult,
};
use std::{fmt::Debug, marker::PhantomData, str, str::FromStr};

#[derive(Clone, Eq, PartialEq)]
pub struct SetupInfo<D: Device = SpectrumAnalyzer> {
    pub main_module_model: Model,
    pub expansion_module_model: Model,
    pub firmware_version: String,
    marker: PhantomData<D>,
}

impl<D: Device> SetupInfo<D> {
    pub fn new(
        main_module_model: Model,
        expansion_module_model: Model,
        firmware_version: String,
    ) -> Self {
        SetupInfo {
            main_module_model,
            expansion_module_model,
            firmware_version,
            marker: PhantomData,
        }
    }

    pub(crate) fn parse_from_bytes_with_prefix<'a>(
        bytes: &'a [u8],
        prefix: &'static [u8],
    ) -> IResult<&'a [u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(prefix)(bytes)?;

        // Parse the main model
        let (bytes, main_model) =
            map_res(map_res(take(3u8), str::from_utf8), Model::from_str)(bytes)?;

        if main_model == Model::None {
            return Err(nom::Err::Error(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            )));
        }

        let (bytes, _) = tag(",")(bytes)?;

        // Parse the expansion model
        let (bytes, exp_model) =
            map_res(map_res(take(3u8), str::from_utf8), Model::from_str)(bytes)?;

        let (bytes, _) = tag(",")(bytes)?;

        // Parse the firmware version
        let (bytes, fw_version) =
            map(map_res(not_line_ending, str::from_utf8), str::to_string)(bytes)?;

        // Consume \r or \r\n line ending and make sure there aren't any bytes left
        let (bytes, _) = all_consuming(opt(line_ending))(bytes)?;

        Ok((bytes, Self::new(main_model, exp_model, fw_version)))
    }
}

impl<D: Device> Debug for SetupInfo<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetupInfo")
            .field("main_module_model", &self.main_module_model)
            .field("expansion_module_model", &self.expansion_module_model)
            .field("firmware_version", &self.firmware_version)
            .finish()
    }
}
