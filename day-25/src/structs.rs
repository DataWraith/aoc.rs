use petgraph::graph::UnGraph;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub graph: UnGraph<String, ()>,
}
