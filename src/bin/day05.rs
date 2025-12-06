use std::ops::RangeInclusive;

use anyhow::Result;
use aoc2025::util::parse_unsigned;
use nom::{
    IResult, Parser as _,
    bytes::tag,
    character::complete::newline,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    all_consuming(separated_pair(
        terminated(
            separated_list1(
                newline,
                separated_pair(parse_unsigned, tag("-"), parse_unsigned)
                    .map(|(start, end)| start..=end),
            ),
            newline,
        ),
        newline,
        terminated(separated_list1(newline, parse_unsigned), newline),
    ))
    .parse(input)
}

fn merge_ranges(
    mut ranges: Vec<RangeInclusive<u64>>,
    mut new_range: RangeInclusive<u64>,
) -> Vec<RangeInclusive<u64>> {
    ranges.retain(|range| {
        let mut retain = true;
        if range.contains(new_range.start()) {
            new_range = *range.start()..=*new_range.end();
            retain = false;
        }
        if range.contains(new_range.end()) {
            new_range = *new_range.start()..=*range.end();
            retain = false;
        }
        if new_range.contains(range.start()) && new_range.contains(range.end()) {
            retain = false;
        }
        retain
    });
    ranges.push(new_range);
    ranges
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day05.txt");
    let (fresh_ranges, ingredients) = parse_input(input)?.1;
    let fresh_ranges = fresh_ranges.into_iter().fold(Vec::new(), merge_ranges);

    let part_1 = ingredients
        .iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(id)))
        .count();
    let part_2: u64 = fresh_ranges
        .into_iter()
        .map(|range| range.count() as u64)
        .sum();
    println!("Day 5, part 1: {part_1}");
    println!("Day 5, part 2: {part_2}");
    Ok(())
}
