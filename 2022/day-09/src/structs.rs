use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub moves: Vec<(Direction, usize)>,
}
