#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub disk: Vec<u64>,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput {
        disk: input
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect(),
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
