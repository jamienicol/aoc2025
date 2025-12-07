use anyhow::{Context, Result, bail};
use ndarray::Array2;

// Returns start position, and a grid where true indicates there is a splitter
// at that location.
fn parse_input(input: &str) -> Result<((usize, usize), Array2<bool>)> {
    let width = input
        .lines()
        .next()
        .map(|line| line.len())
        .context("Empty input")?;
    let height = input.lines().count();

    let mut splitters = Array2::default((width, height));
    let mut start = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                'S' if start.is_some() => bail!("Multiple start positions found"),
                'S' => start = Some((x, y)),
                '^' => splitters[(x, y)] = true,
                _ => bail!("Invalid char at {x}, {y}: '{c}'"),
            }
        }
    }
    let start = start.context("No start position found")?;

    Ok((start, splitters))
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day07.txt");
    let (start, splitters) = parse_input(input)?;

    // The number of timelines which produce a beam at each position in the
    // current row.
    let mut beams = vec![0; splitters.ncols()];
    beams[start.0] = 1;

    let (num_splits, beams) = splitters.columns().into_iter().fold(
        (0, beams),
        |(mut num_splits, prev_beams), splitters| {
            let mut new_beams = vec![0; prev_beams.len()];
            for (x, is_splitter) in splitters.iter().enumerate() {
                if prev_beams[x] > 0 {
                    if *is_splitter {
                        num_splits += 1;
                        if let Some(beam) = new_beams.get_mut(x - 1) {
                            *beam += prev_beams[x];
                        }
                        if let Some(beam) = new_beams.get_mut(x + 1) {
                            *beam += prev_beams[x];
                        }
                    } else {
                        new_beams[x] += prev_beams[x];
                    }
                }
            }
            (num_splits, new_beams)
        },
    );

    println!("Day 7, part 1: {num_splits}");
    println!("Day 7, part 2: {}", beams.iter().sum::<u64>());

    Ok(())
}
