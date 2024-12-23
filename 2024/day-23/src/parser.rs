use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub connections: HashMap<String, HashSet<String>>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        connections
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        connections
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    PuzzleInput { connections }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
