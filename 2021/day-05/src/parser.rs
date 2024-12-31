use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub lines: Vec<(Coordinate<i64>, Coordinate<i64>)>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let coords = parse_ints(input);
    let coords = coords
        .chunks(4)
        .map(|chunk| {
            let x0 = chunk[0];
            let y0 = chunk[1];
            let x1 = chunk[2];
            let y1 = chunk[3];

            (Coordinate::new(x0, y0), Coordinate::new(x1, y1))
        })
        .collect_vec();

    PuzzleInput { lines: coords }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
