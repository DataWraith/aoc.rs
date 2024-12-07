use utility_belt::prelude::parse_ints;
use winnow::Parser;

use crate::structs::*;

pub fn part1(input: &str) -> PuzzleInput {
    let equations: Vec<(i64, Vec<i64>)> = input
        .lines()
        .map(|line| {
            let ints = parse_ints(line);
            (ints[0], ints[1..].to_vec())
        })
        .collect();

    PuzzleInput { equations }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
