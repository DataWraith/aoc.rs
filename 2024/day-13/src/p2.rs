use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    crate::p1::part1(&PuzzleInput {
        games: input.games.clone(),
        part2: true,
    })
}
