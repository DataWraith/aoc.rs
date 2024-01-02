use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let instr = line.split_once(' ');

        if instr.is_none() {
            // Must be a noop, which takes 1 cycle and adds no delta
            instructions.push((1, 0));
            continue;
        }

        // The only other instruction is addx, which takes 2 cycles and adds the
        // given delta
        let (_op, delta) = instr.unwrap();
        let delta = delta.parse::<isize>().unwrap();

        instructions.push((2, delta));
    }

    PuzzleInput { instructions }
}
