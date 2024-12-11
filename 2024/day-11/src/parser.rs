#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub stones: String,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput {
        stones: input.to_string(),
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
