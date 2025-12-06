use anyhow::{Context as _, Result, bail};
use itertools::Itertools;
use ndarray::Array2;
use nonmax::NonMaxU8;

enum Op {
    Add,
    Mul,
}

struct Problem {
    digits: Array2<Option<NonMaxU8>>,
    op: Op,
}

impl Problem {
    fn rows(&self) -> impl Iterator<Item = u64> {
        self.digits.rows().into_iter().map(|row| {
            row.iter()
                .flatten()
                .fold(0, |acc, digit| acc * 10 + digit.get() as u64)
        })
    }

    fn cols(&self) -> impl Iterator<Item = u64> {
        self.digits.columns().into_iter().map(|col| {
            col.iter()
                .flatten()
                .fold(0, |acc, digit| acc * 10 + digit.get() as u64)
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Problem>> {
    let input_width = input.lines().next().context("Empty input")?.len();
    let input_height = input.lines().count();
    let buf = input.chars().collect_vec();
    let grid = Array2::from_shape_vec((input_height, input_width + 1), buf)?;

    let mut problem_offset = 0;
    let mut problems = Vec::new();
    for x in 0..=input_width {
        if grid.column(x).iter().all(|c| c.is_whitespace()) {
            let op = match grid.row(grid.nrows() - 1)[problem_offset] {
                '+' => Op::Add,
                '*' => Op::Mul,
                c => bail!("Unexpected op '{c}'"),
            };
            let digits = grid
                .slice(ndarray::s![..(input_height - 1), problem_offset..x])
                .mapv(|c| match c {
                    ' ' => None,
                    c => c.to_digit(10).and_then(|d| NonMaxU8::new(d as u8)),
                });

            problems.push(Problem { digits, op });
            problem_offset = x + 1;
        }
    }
    Ok(problems)
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day06.txt");
    let problems = parse_input(input)?;

    let part_1 = problems
        .iter()
        .map(|problem| match problem.op {
            Op::Add => problem.rows().sum::<u64>(),
            Op::Mul => problem.rows().product(),
        })
        .sum::<u64>();

    let part_2 = problems
        .iter()
        .map(|problem| match problem.op {
            Op::Add => problem.cols().sum::<u64>(),
            Op::Mul => problem.cols().product(),
        })
        .sum::<u64>();

    println!("Day 6, part 1: {part_1}");
    println!("Day 6, part 2: {part_2}");
    Ok(())
}
