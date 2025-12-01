use nom::{IResult, Parser, character::complete::digit1, combinator::map_res};
use std::str::FromStr;

pub fn parse_unsigned<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse::<T>()).parse(input)
}
