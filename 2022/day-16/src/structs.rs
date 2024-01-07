use utility_belt::prelude::petgraph::*;
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub valve_ids: HashMap<String, NodeIndex<u8>>,
    pub valve_pressures: Vec<(NodeIndex<u8>, u16)>,
    pub network: UnGraph<u16, u8, u8>,
    pub distances: std::collections::HashMap<(NodeIndex<u8>, NodeIndex<u8>), u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time_left: u8,
    pub position: NodeIndex<u8>,
    pub opened: Set32,
    pub pressure_released: u16,
    pub open_valves: u16,
}
