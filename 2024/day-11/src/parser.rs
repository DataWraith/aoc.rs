#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub stones: Vec<u64>,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput {
        stones: input
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
