use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub plan: Vec<PlanEdge>,
}

#[derive(Clone, Debug)]
pub struct PlanEdge {
    pub direction: Direction,
    pub distance: usize,
    pub color: (u8, u8, u8),
}
