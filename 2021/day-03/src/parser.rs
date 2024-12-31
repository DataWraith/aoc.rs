#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub report: Vec<&'static str>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut report = Vec::new();

    for row in input.lines() {
        report.push(row);
    }

    PuzzleInput { report }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
