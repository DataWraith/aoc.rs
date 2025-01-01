use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct Segments {
    pub calibration: Vec<String>,
    pub output: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub segments: Vec<Segments>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut segments = Vec::new();

    for line in input.lines() {
        let (calibration, output) = line.split_once(" | ").unwrap();
        segments.push(Segments {
            calibration: calibration
                .split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .collect(),
            output: output
                .split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .collect(),
        });
    }

    PuzzleInput { segments }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
