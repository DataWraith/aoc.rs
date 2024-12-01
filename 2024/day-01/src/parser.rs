use winnow::{PResult, Parser, combinator::trace};

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let lines: Vec<&str> = input.lines().collect();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    PuzzleInput { left, right }
}
