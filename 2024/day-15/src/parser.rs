use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub warehouse: Grid2D<char>,
    pub robot_moves: Vec<char>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let (warehouse, robot_moves) = input.split_once("\n\n").unwrap();
    let warehouse = format!("{}\n", warehouse);
    let warehouse: Grid2D<char> = (warehouse.as_str()).into();

    PuzzleInput {
        warehouse,
        robot_moves: robot_moves.chars().filter(|c| *c != '\n').collect(),
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    let (warehouse, robot_moves) = input.split_once("\n\n").unwrap();
    let warehouse = format!("{}\n", warehouse);
    let warehouse = warehouse
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");

    PuzzleInput {
        warehouse: (warehouse.as_str()).into(),
        robot_moves: robot_moves.chars().filter(|c| *c != '\n').collect(),
    }
}
