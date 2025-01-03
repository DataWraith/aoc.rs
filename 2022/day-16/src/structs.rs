use utility_belt::prelude::*;

use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub valve_ids: HashMap<String, NodeIndex<u8>>,
    pub valve_pressures: Vec<(NodeIndex<u8>, u16)>,
    pub network: UnGraph<u16, u8, u8>,
    pub distances: HashMap<(NodeIndex<u8>, NodeIndex<u8>), u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time_left: u8,
    pub position: NodeIndex<u8>,
    pub opened: MiniBitset<u16>,
    pub pressure_released: u16,
    pub open_valves: u16,
}
