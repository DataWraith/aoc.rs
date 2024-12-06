
use crate::structs::*;

pub fn part1(input: &str) -> PuzzleInput {
    // Checklist:
    //
    // 1. Can this be parsed using parse_ints(input)
    // 2. Can this be parsed using a regular expression
    // 3. Winnow parser?
    // 4. ???

    PuzzleInput { grid: input.into() }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
