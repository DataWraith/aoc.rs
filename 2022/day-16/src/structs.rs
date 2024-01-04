use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

use utility_belt::prelude::petgraph::*;
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub valve_ids: HashMap<String, NodeIndex<u32>>,
    pub valve_pressures: Vec<(NodeIndex<u32>, u32)>,
    pub network: UnGraph<u32, u32>,
    pub distances: std::collections::HashMap<(NodeIndex<u32>, NodeIndex<u32>), u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time_left: u32,
    pub position: NodeIndex<u32>,
    pub opened: Set64,
    pub pressure_released: u32,
    pub open_valves: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub struct Set64(u64);

impl Set64 {
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    pub fn contains(&self, i: usize) -> bool {
        assert!(i < 64);

        self.0 & (1 << i) != 0
    }

    pub fn insert(&mut self, i: usize) {
        assert!(i < 64);

        self.0 |= 1 << i
    }

    pub fn remove(&mut self, i: usize) {
        assert!(i < 64);
        self.0 &= !(1 << i)
    }
}
