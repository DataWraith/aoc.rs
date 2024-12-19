use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub patterns: HashSet<String>,
    pub desired_designs: Vec<String>,
    pub longest_pattern: usize,
}

pub fn part1(input: &str) -> PuzzleInput {
    let (patterns, desired_designs) = input.split_once("\n\n").unwrap();
    let patterns: HashSet<String> = patterns.split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = desired_designs
        .trim_end()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let longest_pattern = patterns.iter().map(|s| s.len()).max().unwrap();

    PuzzleInput {
        patterns,
        desired_designs: designs,
        longest_pattern,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
