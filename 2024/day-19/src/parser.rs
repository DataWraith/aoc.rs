#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub patterns: Vec<String>,
    pub desired_designs: Vec<String>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let (patterns, desired_designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<String> = patterns.split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = desired_designs
        .trim_end()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    PuzzleInput {
        patterns,
        desired_designs: designs,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
