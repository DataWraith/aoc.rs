use utility_belt::prelude::Grid2D;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub patterns: Vec<MirrorPattern>,
}

pub type MirrorPattern = Grid2D<char>;
