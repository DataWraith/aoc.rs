use std::collections::BTreeMap;

use petgraph::{graph::DiGraph, visit::EdgeRef, Direction::Incoming};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub graph: DiGraph<String, ()>,
    pub node_types: BTreeMap<String, char>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub button_presses: usize,
    pub s: BTreeMap<String, ModuleState>,
    pub conjunction_cycles: BTreeMap<String, usize>,
}

impl State {
    pub fn new(input: &PuzzleInput) -> Self {
        let mut state = BTreeMap::new();

        for node in input.graph.node_indices() {
            let module = &input.graph[node];

            match input.node_types.get(module) {
                Some('b') => {
                    state.insert(module.clone(), ModuleState::Broadcaster);
                }
                Some('%') => {
                    state.insert(module.clone(), ModuleState::FlipFlop { on: false });
                }
                Some('&') => {
                    let mut memory = BTreeMap::default();

                    for source in input.graph.edges_directed(node, Incoming) {
                        let n = &input.graph[source.source()];
                        memory.insert(n.clone(), Pulse::Low);
                    }

                    state.insert(module.clone(), ModuleState::Conjunction { memory });
                }
                _ => {}
            }
        }

        Self {
            button_presses: 0,
            s: state,
            conjunction_cycles: BTreeMap::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModuleState {
    Broadcaster,
    FlipFlop { on: bool },
    Conjunction { memory: BTreeMap<String, Pulse> },
}
