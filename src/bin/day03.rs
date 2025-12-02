use anyhow::Result;
use nom::{
    IResult, Parser as _,
    character::complete::{newline, one_of},
    combinator::all_consuming,
    multi::{many1, separated_list0},
    sequence::terminated,
};

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    all_consuming(terminated(
        separated_list0(
            newline,
            many1(one_of("0123456789").map(|c| c.to_digit(10).unwrap() as u64)),
        ),
        newline,
    ))
    .parse(input)
}

trait PositionMaxFirst: Iterator {
    /// Like [`itertools::Itertools::position_max()`], but if several elements are equally
    /// maximum, the position of the first of them is returned, rather than the last.
    fn position_max_first(self) -> Option<usize>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.enumerate()
            .max_by(|(x_pos, x_val), (y_pos, y_val)| (x_val, y_pos).cmp(&(y_val, x_pos)))
            .map(|(pos, _val)| pos)
    }
}
impl<T: Iterator> PositionMaxFirst for T {}

fn max_joltage(bank: &[u64], num_digits: usize) -> u64 {
    (0..num_digits)
        .rev()
        .fold((0, bank), |(mut joltage, bank), digit_idx| {
            let pos = bank[..(bank.len() - digit_idx)]
                .iter()
                .position_max_first()
                .unwrap();
            joltage += bank[pos] * 10u64.pow(digit_idx as u32);

            (joltage, &bank[(pos + 1)..])
        })
        .0
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day03.txt");
    let banks = parse_input(input)?.1;

    let part_1: u64 = banks.iter().map(|bank| max_joltage(bank, 2)).sum();
    let part_2: u64 = banks.iter().map(|bank| max_joltage(bank, 12)).sum();

    println!("Day 3, part 1: {part_1}");
    println!("Day 3, part 2: {part_2}");
    Ok(())
}
