use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub lanternfish: Vec<u8>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput {
        lanternfish: parse_uints(input).into_iter().map(|x| x as u8).collect(),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
