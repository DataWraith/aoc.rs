use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    PuzzleInput {
        heightmap: Grid2D::parse(input),
    }
}
