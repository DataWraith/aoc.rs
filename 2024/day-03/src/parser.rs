use regex::Regex;

use crate::structs::*;

pub fn parse2(input: &str) -> PuzzleInput {
    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\(\d\d?\d?,\d\d?\d?\)").unwrap();

    let instructions = re.captures_iter(input).collect::<Vec<_>>();
    let mut muls = Vec::new();

    let mut enabled = true;

    for instr in instructions {
        if &instr[0] == "do()" {
            enabled = true;
        } else if &instr[0] == "don't()" {
            enabled = false;
        } else if enabled {
            let mul = parse(&instr[0]);
            muls.push(mul.muls[0]);
        }
    }

    PuzzleInput { muls }
}

pub fn parse(input: &str) -> PuzzleInput {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let muls = re
        .captures_iter(input)
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    PuzzleInput { muls }
}
