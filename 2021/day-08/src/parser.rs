use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct Segments<'a> {
    pub calibration: Vec<&'a str>,
    pub output: Vec<&'a str>,
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub segments: Vec<Segments<'static>>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut segments = Vec::new();

    for line in input.lines() {
        let (calibration, output) = line.split_once(" | ").unwrap();
        segments.push(Segments {
            calibration: calibration.split_whitespace().collect(),
            output: output.split_whitespace().collect(),
        });
    }

    PuzzleInput { segments }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
