use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub grid: Grid2D<char>,
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Beam {
    pub position: Coordinate,
    pub direction: Direction,
}
