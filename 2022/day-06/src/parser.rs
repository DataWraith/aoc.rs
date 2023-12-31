use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    PuzzleInput {
        sequence: input.chars().collect_vec(),
    }
}
