use utility_belt::prelude::*;

use crate::p1::Board;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub numbers: Vec<i64>,
    pub boards: Vec<Board>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers = parse_ints(numbers);

    let boards: Vec<Grid2D<i64>> = parse_ints(boards)
        .chunks(25)
        .map(|chunk| Grid2D::from_shape_vec(5, 5, chunk.to_vec()))
        .collect();

    PuzzleInput {
        numbers,
        boards: boards
            .into_iter()
            .map(|grid| Board {
                marked: grid.map(|_| false).into(),
                numbers: grid,
            })
            .collect(),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
