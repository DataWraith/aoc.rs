use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    PuzzleInput {
        numbers: input
            .lines()
            .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
            .collect(),
    }
}
