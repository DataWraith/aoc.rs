use regex::Regex;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let muls = re
        .captures_iter(input)
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    dbg!(&muls);

    PuzzleInput { muls }
}
