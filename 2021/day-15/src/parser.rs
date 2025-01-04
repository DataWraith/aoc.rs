use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub grid: Grid2D<u32>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let grid: Grid2D<char> = input.into();
    let grid: Grid2D<u32> = grid.map(|c| c.to_digit(10).unwrap() as u32);

    PuzzleInput { grid }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
"};
