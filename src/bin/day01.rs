use anyhow::Result;
use aoc2025::util::parse_unsigned;
use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{pair, terminated},
};

fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    all_consuming(terminated(
        separated_list0(
            newline,
            pair(one_of("LR"), parse_unsigned).map(|(dir, dist): (char, i32)| match dir {
                'L' => -dist,
                'R' => dist,
                _ => unreachable!(),
            }),
        ),
        newline,
    ))
    .parse(input)
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day01.txt");
    let turns = parse_input(input)?.1;

    let (_, part_1, part_2) = turns.iter().fold(
        (50, 0, 0),
        |(pos, mut stopped_zero, mut passed_zero), turn| {
            let new_pos = (pos + turn).rem_euclid(100);
            stopped_zero += (new_pos == 0) as i32;
            passed_zero += ((pos * turn.signum()).rem_euclid(100) + turn.abs()) / 100;
            (new_pos, stopped_zero, passed_zero)
        },
    );

    println!("Day 1, part 1: {part_1}");
    println!("Day 1, part 2: {part_2}");
    Ok(())
}
