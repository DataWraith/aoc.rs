use crate::structs::*;

use utility_belt::prelude::*;

pub fn parse(input: &str) -> PuzzleInput {
    let ints = parse_ints(input);

    let table = Array2::from_shape_vec((ints.len() / 2, 2), ints).unwrap();

    let left = table.column(0).to_vec();
    let right = table.column(1).to_vec();

    PuzzleInput { left, right }
}
