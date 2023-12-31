use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut result = Vec::new();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        result.push((chars[0].into(), chars[2].into()));
    }

    PuzzleInput { guide: result }
}
