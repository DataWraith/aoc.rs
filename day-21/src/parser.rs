use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    PuzzleInput {
        grid: Grid2D::parse(input),
    }
}
