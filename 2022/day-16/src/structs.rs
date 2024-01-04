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
    pub opened: BTreeSet<NodeIndex<u32>>,
    pub pressure_released: u32,
    pub open_valves: u32,
}
