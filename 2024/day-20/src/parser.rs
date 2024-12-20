use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub maze: Grid2D<char>,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput { maze: input.into() }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
