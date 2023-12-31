use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let split = input.split("\n\n").collect::<Vec<_>>();

    let mut crates: Vec<Vec<char>> = vec![vec![]; 10];
    let mut instructions = Vec::new();

    for line in split[0].lines() {
        let mut idx = 1;

        while let Some(c) = line.get(idx..(idx + 1)) {
            if c == " " {
                idx += 4;
                continue;
            }

            if c.chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
            {
                break;
            }

            crates[1 + (idx - 1) / 4].push(c.chars().next().unwrap());
            idx += 4;
        }
    }

    for line in split[1].lines() {
        let line_split = line.split_ascii_whitespace().collect::<Vec<_>>();

        let n = line_split[1].parse::<usize>().unwrap();
        let f = line_split[3].parse::<usize>().unwrap();
        let t = line_split[5].parse::<usize>().unwrap();

        instructions.push((n, f, t));
    }

    for stack in crates.iter_mut() {
        stack.reverse();
    }

    PuzzleInput {
        crates,
        instructions,
    }
}
