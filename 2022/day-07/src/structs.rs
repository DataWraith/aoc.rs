use std::collections::BTreeMap;

use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub filesystem: BTreeMap<String, usize>,
}
