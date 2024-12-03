use regex::Regex;

use crate::structs::*;

pub fn parse2(input: &str) -> PuzzleInput {
    // Strip out all characters that are preceded by "don't" until the next "do"
    // or the end of the input
    let re = Regex::new(r"(?s)don't\(\).*?(\z|do\(\))").unwrap();
    let input = re.replace_all(input, "");

    // Then just parse the remaining mul calls as per part 1
    parse(&input)
}

pub fn parse(input: &str) -> PuzzleInput {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut muls = vec![];

    for (_cap, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        muls.push(x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap());
    }

    PuzzleInput { muls }
}
