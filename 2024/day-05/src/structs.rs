use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub dependencies: HashMap<u64, HashSet<u64>>,
    pub pages: Vec<Vec<u64>>,
}
