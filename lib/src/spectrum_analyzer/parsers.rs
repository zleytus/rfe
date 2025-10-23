use std::{convert::TryFrom, str::FromStr};

use nom::{combinator::map_res, IResult, Parser};

use super::{CalcMode, Mode};
use crate::rf_explorer::parsers::*;

pub(super) fn parse_amplitude<T: FromStr>(bytes: &[u8]) -> IResult<&[u8], T> {
    num_parser(4u8).parse(bytes)
}

pub(super) fn parse_calc_mode(bytes: &[u8]) -> IResult<&[u8], CalcMode> {
    map_res(num_parser::<u8>(3u8), CalcMode::try_from).parse(bytes)
}

pub(super) fn parse_mode(bytes: &[u8]) -> IResult<&[u8], Mode> {
    map_res(num_parser::<u8>(3u8), Mode::try_from).parse(bytes)
}
