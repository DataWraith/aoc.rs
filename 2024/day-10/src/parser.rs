use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub map: Grid2D<u8>,
}

pub fn part1(input: &str) -> PuzzleInput {
    // Checklist:
    //
    // 1. Can this be parsed using parse_ints(input)
    // 2. Can this be parsed using a regular expression
    // 3. Winnow parser?
    // 4. ???

    let map: Grid2D<char> = input.into();
    let map: Grid2D<u8> = map.map(|c| c.to_digit(10).unwrap() as u8);

    PuzzleInput { map }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
