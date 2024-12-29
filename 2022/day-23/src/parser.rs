use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub cells: HashSet<Coordinate>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let grid: Grid2D<char> = input.into();
    let grid = grid.map(|c| *c == '#');
    let cells = grid
        .iter()
        .filter(|(_coord, square)| **square)
        .map(|(coord, _square)| coord)
        .collect();

    PuzzleInput { cells }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
