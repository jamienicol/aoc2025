use std::ops::RangeInclusive;

use anyhow::Result;
use aoc2025::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::newline,
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{separated_pair, terminated},
};

fn parse_input(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    all_consuming(terminated(
        separated_list0(
            tag(","),
            separated_pair(parse_unsigned, tag("-"), parse_unsigned)
                .map(|(start, end)| start..=end),
        ),
        newline,
    ))
    .parse(input)
}

fn is_invalid(id: u64, allow_many_repeats: bool) -> bool {
    let num_digits = id.ilog10() + 1;
    let min_chunk_size = match allow_many_repeats {
        true => 1,
        false => (num_digits / 2).max(1),
    };
    (min_chunk_size..=(num_digits / 2)).any(|chunk_size| {
        if num_digits % chunk_size != 0 {
            return false;
        }
        (0..(num_digits / chunk_size))
            .map(|chunk| (id / 10u64.pow(chunk * chunk_size)) % 10u64.pow(chunk_size))
            .all_equal()
    })
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day02.txt");
    let ranges = parse_input(input)?.1;

    let part_1 = ranges
        .iter()
        .cloned()
        .flatten()
        .filter(|id| is_invalid(*id, false))
        .sum::<u64>();

    let part_2 = ranges
        .into_iter()
        .flatten()
        .filter(|id| is_invalid(*id, true))
        .sum::<u64>();

    println!("Day 2, part 1: {part_1}");
    println!("Day 2, part 2: {part_2}");
    Ok(())
}
