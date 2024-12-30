use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub grid: Grid2D<char>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput { grid: input.into() }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
