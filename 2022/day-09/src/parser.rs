use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let moves = input
        .lines()
        .map(|line| {
            let (dir, len) = line.split_once(' ').unwrap();
            let direction: Direction = dir.chars().next().unwrap().try_into().unwrap();
            let length = len.parse::<usize>().unwrap();

            (direction, length)
        })
        .collect();

    PuzzleInput { moves }
}
