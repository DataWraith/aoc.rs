use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub numbers: Vec<u64>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let numbers = parse_uints(input);
    PuzzleInput { numbers }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
