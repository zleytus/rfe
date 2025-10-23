use nom::{
    bytes::complete::{tag, take},
    character::complete::line_ending,
    combinator::{all_consuming, map_res, opt},
    error::Error,
    IResult, Parser,
};
use std::str::{self, FromStr};

pub(crate) fn parse_comma(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(",")(bytes)
}

pub(crate) fn parse_opt_line_ending(bytes: &[u8]) -> IResult<&[u8], Option<&[u8]>> {
    all_consuming(opt(line_ending)).parse(bytes)
}

pub(crate) fn parse_num<'a, T>(
    digits: u8,
) -> impl Parser<&'a [u8], Output = T, Error = Error<&'a [u8]>>
where
    T: FromStr,
{
    map_res(map_res(take(digits), str::from_utf8), T::from_str)
}

pub(crate) fn parse_frequency<'a>(
    digits: u8,
) -> impl Parser<&'a [u8], Output = u64, Error = Error<&'a [u8]>> {
    parse_num(digits)
}
