use petgraph::prelude::UnGraphMap;
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub cave: UnGraphMap<&'static str, ()>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut graph = UnGraphMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        graph.add_edge(from, to, ());
    }

    PuzzleInput { cave: graph }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
"};
