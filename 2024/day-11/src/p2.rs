use crate::{p1::blink_many, parser::*};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    blink_many(&input.stones, 75).to_string()
}