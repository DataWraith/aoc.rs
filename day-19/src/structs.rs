use std::ops::Range;

use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub workflows: HashMap<String, Workflow>,
    pub parts: Vec<Part>,
}

#[derive(Clone, Debug)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub default: String,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub category: char,
    pub comparison: char,
    pub value: usize,
    pub next: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Flow {
    pub part: Part,
    pub current_workflow: String,
    pub current_index: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    pub fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PartRange {
    pub x: Range<usize>,
    pub m: Range<usize>,
    pub a: Range<usize>,
    pub s: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFlow {
    pub part: PartRange,
    pub current_workflow: String,
    pub current_index: usize,
    pub accepted: usize,
}
