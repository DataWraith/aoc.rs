use utility_belt::prelude::parse_ints;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let reports = input.lines().map(parse_ints).collect();

    PuzzleInput { reports }
}
