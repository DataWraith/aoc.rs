use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub grids: Vec<Grid2D<char>>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let grids = input.split("\n\n").map(|grid| grid.into()).collect();
    PuzzleInput { grids }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
