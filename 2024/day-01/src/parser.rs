use crate::structs::*;

use utility_belt::prelude::*;

pub fn parse(input: &str) -> PuzzleInput {
    let ints = parse_ints(input);

    let left = ints.iter().cloned().step_by(2).collect();
    let right = ints.iter().cloned().skip(1).step_by(2).collect();

    PuzzleInput { left, right }
}
