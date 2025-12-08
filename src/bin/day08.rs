use std::collections::HashSet;

use anyhow::Result;
use aoc2025::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    IResult, Parser as _,
    character::complete::{char, newline},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
};

type Pos = (u64, u64, u64);

fn parse_input(input: &str) -> IResult<&str, Vec<Pos>> {
    all_consuming(terminated(
        separated_list1(
            newline,
            (
                terminated(parse_unsigned, char(',')),
                terminated(parse_unsigned, char(',')),
                parse_unsigned,
            ),
        ),
        newline,
    ))
    .parse(input)
}

fn connect_boxes(a: &Pos, b: &Pos, circuits: &mut Vec<HashSet<Pos>>) {
    let circuit_a_idx = circuits
        .iter()
        .position(|circuit| circuit.contains(a))
        .unwrap();
    let circuit_b_idx = circuits
        .iter()
        .position(|circuit| circuit.contains(b))
        .unwrap();
    if circuit_a_idx != circuit_b_idx {
        let circuit_b = circuits[circuit_b_idx].clone();
        circuits[circuit_a_idx].extend(circuit_b);
        circuits.swap_remove(circuit_b_idx);
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day08.txt");
    let boxes = parse_input(input)?.1;

    let mut connections = boxes
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| {
            std::cmp::Reverse(
                a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2),
            )
        })
        .collect_vec();
    let mut circuits = boxes
        .iter()
        .map(|b| std::iter::once(*b).collect::<HashSet<Pos>>())
        .collect_vec();

    for (box_a, box_b) in connections.drain((connections.len() - 1000)..) {
        connect_boxes(box_a, box_b, &mut circuits);
    }
    let part_1: usize = circuits
        .iter()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product();

    let part_2 = connections
        .into_iter()
        .rev()
        .find(|(box_a, box_b)| {
            connect_boxes(box_a, box_b, &mut circuits);
            circuits.len() == 1
        })
        .map(|(box_a, box_b)| box_a.0 * box_b.0)
        .unwrap();

    println!("Day 8, part 1: {part_1}");
    println!("Day 8, part 2: {part_2}");

    Ok(())
}
