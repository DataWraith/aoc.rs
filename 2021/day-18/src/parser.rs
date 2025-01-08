use std::str::FromStr;

use utility_belt::prelude::*;

use winnow::{ascii::dec_uint, combinator::alt, prelude::*};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnailfishNumber {
    Literal(u64),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl FromStr for SnailfishNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        snailfish_number.parse(s).map_err(|_| ())
    }
}

fn snailfish_literal(input: &mut &str) -> PResult<SnailfishNumber> {
    dec_uint(input).map(|num| SnailfishNumber::Literal(num))
}

fn snailfish_number(input: &mut &str) -> PResult<SnailfishNumber> {
    (
        "[",
        alt((snailfish_literal, snailfish_number)),
        ",",
        alt((snailfish_literal, snailfish_number)),
        "]",
    )
        .parse_next(input)
        .map(|(_, left, _, right, _)| SnailfishNumber::Pair(Box::new(left), Box::new(right)))
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub numbers: Vec<SnailfishNumber>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut numbers = Vec::new();

    for line in input.lines() {
        numbers.push(line.trim().parse::<SnailfishNumber>().unwrap());
    }

    PuzzleInput { numbers }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"};
