use utility_belt::prelude::petgraph::*;
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub valve_ids: HashMap<String, NodeIndex<u32>>,
    pub network: UnGraph<u32, u32>,
}
