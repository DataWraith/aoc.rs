use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub map: Grid2D<u8>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let grid: Grid2D<char> = input.into();

    PuzzleInput {
        map: grid.map(|c| c.to_digit(10).unwrap() as u8),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
