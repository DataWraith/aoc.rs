use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub rules: HashSet<(u32, u32)>,
    pub pages: Vec<Vec<u32>>,
}
