use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    PuzzleInput {
        sequence: input
            .lines()
            .join("")
            .split(',')
            .map(|s| s.to_string())
            .collect(),
    }
}
