use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    crate::p1::part1(&PuzzleInput {
        games: input.games.clone(),
        offset: 10000000000000i64,
    })
}
