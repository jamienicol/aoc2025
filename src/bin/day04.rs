use anyhow::{Result, ensure};
use itertools::Itertools;
use ndarray::Array2;

fn parse_input(input: &str) -> Result<Array2<bool>> {
    let width = input.lines().next().map(|row| row.len()).unwrap_or(0);
    let height = input.lines().count();
    let mut grid = Array2::default((width, height));
    for (y, line) in input.lines().enumerate() {
        ensure!(line.len() == width);
        for (x, c) in line.chars().enumerate() {
            grid[(x, y)] = c == '@';
        }
    }

    Ok(grid)
}

pub fn adjacent_positions(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    ((x.max(1) - 1)..=(x + 1).min(width - 1))
        .cartesian_product((y.max(1) - 1)..=(y + 1).min(height - 1))
        .filter(move |adj| *adj != (x, y))
}

fn removable_rolls(rolls: &Array2<bool>) -> impl Iterator<Item = (usize, usize)> {
    (0..rolls.ncols())
        .cartesian_product(0..rolls.nrows())
        .filter(|pos| {
            rolls[*pos]
                && adjacent_positions(*pos, rolls.dim())
                    .filter(|pos| rolls[*pos])
                    .count()
                    < 4
        })
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day04.txt");
    let mut rolls = parse_input(input)?;

    let part_1 = removable_rolls(&rolls).count();

    let mut part_2 = 0;
    loop {
        let to_remove = removable_rolls(&rolls).collect_vec();
        if to_remove.is_empty() {
            break;
        }
        part_2 += to_remove.len();
        to_remove.into_iter().for_each(|pos| rolls[pos] = false);
    }

    println!("Day 4, part 1: {part_1}");
    println!("Day 4, part 2: {part_2}");
    Ok(())
}
