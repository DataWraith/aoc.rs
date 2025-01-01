use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub octopi: Grid2D<u16>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let char_grid: Grid2D<char> = input.into();
    let octopi: Grid2D<u16> = char_grid.map(|c| c.to_digit(10).unwrap() as u16);

    PuzzleInput { octopi }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
"};
