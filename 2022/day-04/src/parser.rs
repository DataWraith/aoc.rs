use std::ops::RangeInclusive;

use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut pair_assignments = Vec::new();

    for line in input.lines() {
        let assignments = line
            .replace('-', ",")
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>();

        let start1 = assignments[0].parse::<usize>().unwrap();
        let end1 = assignments[1].parse::<usize>().unwrap();
        let start2 = assignments[2].parse::<usize>().unwrap();
        let end2 = assignments[3].parse::<usize>().unwrap();

        pair_assignments.push((
            RangeInclusive::new(start1, end1),
            RangeInclusive::new(start2, end2),
        ))
    }

    PuzzleInput {
        assignments: pair_assignments,
    }
}
