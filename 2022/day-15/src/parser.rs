use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub sensors: Vec<(Coordinate, Coordinate)>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let ints = parse_ints(input);
    let sensors = ints
        .chunks(4)
        .map(|chunk| {
            (
                Coordinate::new(chunk[0] as i32, chunk[1] as i32),
                Coordinate::new(chunk[2] as i32, chunk[3] as i32),
            )
        })
        .collect();
    PuzzleInput { sensors }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
