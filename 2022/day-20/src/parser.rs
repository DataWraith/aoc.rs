use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub sequence: Vec<i64>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput {
        sequence: parse_ints(input),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
