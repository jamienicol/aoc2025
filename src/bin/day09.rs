use std::ops::RangeInclusive;

use anyhow::{Context, Result};
use aoc2025::util::parse_unsigned;
use itertools::Itertools;
use nom::{
    IResult, Parser as _,
    bytes::tag,
    character::complete::newline,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

struct Pos {
    x: u64,
    y: u64,
}

struct Line {
    pos: u64,
    range: RangeInclusive<u64>,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pos>> {
    all_consuming(terminated(
        separated_list1(
            newline,
            separated_pair(parse_unsigned, tag(","), parse_unsigned).map(|(x, y)| Pos { x, y }),
        ),
        newline,
    ))
    .parse(input)
}

fn rect_area(a: &Pos, b: &Pos) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

// Returns tuple of the rect's horizontal and vertical border lines
fn rect_borders(a: &Pos, b: &Pos) -> ([Line; 2], [Line; 2]) {
    let min = Pos {
        x: a.x.min(b.x),
        y: a.y.min(b.y),
    };
    let max = Pos {
        x: a.x.max(b.x),
        y: a.y.max(b.y),
    };
    (
        [
            Line {
                pos: min.y,
                range: (min.x + 1)..=(max.x - 1),
            },
            Line {
                pos: max.y,
                range: (min.x + 1)..=(max.x - 1),
            },
        ],
        [
            Line {
                pos: min.x,
                range: (min.y + 1)..=(max.y - 1),
            },
            Line {
                pos: max.x,
                range: (min.y + 1)..=(max.y - 1),
            },
        ],
    )
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day09.txt");
    let red_tiles = parse_input(input)?.1;

    let part_1 = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| rect_area(a, b))
        .max()
        .context("Empty list")?;

    let (h_lines, v_lines) = red_tiles.iter().circular_tuple_windows().fold(
        (Vec::new(), Vec::new()),
        |(mut h_lines, mut v_lines), (a, b)| {
            if a.x == b.x {
                v_lines.push(Line {
                    pos: a.x,
                    range: a.y.min(b.y)..=a.y.max(b.y),
                });
            } else if a.y == b.y {
                h_lines.push(Line {
                    pos: a.y,
                    range: a.x.min(b.x)..=a.x.max(b.x),
                });
            }
            (h_lines, v_lines)
        },
    );

    let part_2 = red_tiles
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| std::cmp::Reverse(rect_area(a, b)))
        .find(|(a, b)| {
            let (rect_h_borders, rect_v_borders) = rect_borders(a, b);
            !h_lines
                .iter()
                .cartesian_product(&rect_v_borders)
                .chain(v_lines.iter().cartesian_product(&rect_h_borders))
                .any(|(a, b)| a.range.contains(&b.pos) && b.range.contains(&a.pos))
        })
        .map(|(a, b)| rect_area(a, b))
        .context("Couldn't find rectangle")?;

    println!("Day 9, part 1: {part_1}");
    println!("Day 9, part 2: {part_2}");
    Ok(())
}
