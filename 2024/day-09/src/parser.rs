use winnow::{combinator::trace, stream::AsChar, PResult, Parser};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub disk: Vec<u8>,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput {
        disk: input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8) )
            .collect(),
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
