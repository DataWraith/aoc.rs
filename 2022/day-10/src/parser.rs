use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let instr = line.split_once(" ");

        if instr.is_none() {
            // Must be a noop, which takes 1 cycle and adds no delta
            instructions.push((1, 0));
            continue;
        }

        let (op, arg) = instr.unwrap();

        let delta = arg.parse::<isize>().unwrap();

        instructions.push((2, delta));
    }

    PuzzleInput { instructions }
}
