use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub graph: UnGraphMap<&'static str, ()>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut graph = UnGraphMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        graph.add_edge(a, b, ());
    }

    PuzzleInput { graph }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
