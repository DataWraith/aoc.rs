use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub report: Grid2D<bool>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let grid: Grid2D<char> = input.into();
    let grid: Grid2D<bool> = grid.map(|c| *c == '1');

    PuzzleInput { report: grid }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
