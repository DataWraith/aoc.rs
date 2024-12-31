use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub lanternfish: Vec<u64>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput {
        lanternfish: parse_uints(input),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
