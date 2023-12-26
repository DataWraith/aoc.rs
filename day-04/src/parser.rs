use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let cards = input.lines().map(Card::from).collect();

    PuzzleInput { cards }
}
