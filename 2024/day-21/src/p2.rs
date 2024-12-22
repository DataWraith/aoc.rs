use utility_belt::prelude::*;

use crate::{
    p1::{solve, CodePad},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let codepad = CodePad::new_codepad();
    let dirpad = CodePad::new_dirpad();

    let mut sum = 0;

    for code in input.codes.iter() {
        let solution = solve(code, &codepad, &dirpad, 25);

        let num = parse_uints(code)[0];
        sum += num as usize * solution;
    }

    sum.to_string()
}
