use std::collections::LinkedList;

use utility_belt::prelude::*;

use crate::{
    p1::{blink, blink_many},
    parser::*,
};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    blink_many(&mut input.stones.iter().join(" "), 75).to_string()
}
