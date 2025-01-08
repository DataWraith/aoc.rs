use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub target_area: (Coordinate, Coordinate),
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let ints = parse_ints(input);

    let top_left = Coordinate::new(ints[0] as i32, ints[2] as i32);
    let bottom_right = Coordinate::new(ints[1] as i32, ints[3] as i32);

    PuzzleInput {
        target_area: (top_left, bottom_right),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    target area: x=20..30, y=-10..-5
"};
