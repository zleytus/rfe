use std::convert::TryFrom;

use nom::{combinator::map_res, IResult, Parser};

use super::{Attenuation, PowerLevel, RfPower};
use crate::rf_explorer::parsers::*;

pub(super) fn parse_attenuation(bytes: &[u8]) -> IResult<&[u8], Attenuation> {
    map_res(num_parser::<u8>(1u8), Attenuation::try_from).parse(bytes)
}

pub(super) fn parse_power_level(bytes: &[u8]) -> IResult<&[u8], PowerLevel> {
    map_res(num_parser::<u8>(1u8), PowerLevel::try_from).parse(bytes)
}

pub(super) fn parse_sweep_delay_ms(bytes: &[u8]) -> IResult<&[u8], u16> {
    num_parser(5u8).parse(bytes)
}

pub(super) fn parse_rf_power(bytes: &[u8]) -> IResult<&[u8], RfPower> {
    map_res(num_parser::<u8>(1u8), RfPower::try_from).parse(bytes)
}
