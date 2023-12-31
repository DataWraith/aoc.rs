use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub assignments: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>,
}
