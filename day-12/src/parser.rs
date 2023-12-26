use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    PuzzleInput {
        records: input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .map(|split| {
                (
                    split[0].clone(),
                    split[1].split(',').map(|s| s.parse().unwrap()).collect(),
                )
            })
            .collect(),
    }
}
