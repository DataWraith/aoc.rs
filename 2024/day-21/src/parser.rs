use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub codes: Vec<String>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let codes = input.lines().map(|line| line.to_string()).collect();

    PuzzleInput { codes }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
