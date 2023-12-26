#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub races: Vec<Race>,
}

#[derive(Debug, Clone)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}
