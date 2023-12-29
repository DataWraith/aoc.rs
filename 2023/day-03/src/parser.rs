use crate::structs::*;
use utility_belt::prelude::*;

pub fn parse(input: &str) -> PuzzleInput {
    let grid = Grid2D::parse(input);

    PuzzleInput { grid }
}
