use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub positions: Vec<i64>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput {
        positions: parse_ints(input),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
