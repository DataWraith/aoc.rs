use utility_belt::grid::Grid2D;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub garden: Grid2D<char>,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput {
        garden: input.into(),
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
