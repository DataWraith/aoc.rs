use std::{collections::BTreeMap, path::PathBuf};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub filesystem: BTreeMap<PathBuf, usize>,
}
