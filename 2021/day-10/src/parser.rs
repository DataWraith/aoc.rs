#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub lines: Vec<&'static str>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput {
        lines: input.lines().map(|s| s.trim()).collect(),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
