use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub grid: Grid2D<HeatLoss>,
}

#[derive(Clone, Debug)]
pub struct HeatLoss(pub u8);

impl From<char> for HeatLoss {
    fn from(c: char) -> Self {
        Self(c as u8 - b'0')
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Crucible {
    pub position: Coordinate,
    pub direction: Option<Direction>,
    pub cur_straight: usize,
    pub max_straight: usize,
    pub min_straight: usize,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
