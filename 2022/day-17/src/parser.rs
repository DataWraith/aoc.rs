use utility_belt::prelude::*;

const ROCKS: &str = indoc! {"
####
....

.#.
###
.#.

###
..#
..#

#
#
#
#

##
##
"};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub rocks: Vec<BoolGrid2D>,
    pub jets: Vec<char>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let rocks: Vec<BoolGrid2D> = ROCKS
        .split("\n\n")
        .map(Grid2D::<char>::from)
        .map(|g| g.map(|c| *c == '#'))
        .map(|g| g.into())
        .collect();

    let jets: Vec<char> = input.trim().chars().collect();

    PuzzleInput { rocks, jets }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
