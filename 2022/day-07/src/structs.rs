use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub filesystem: BTreeMap<String, usize>,
}
