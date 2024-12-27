use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub cubes: HashSet<(i32, i32, i32)>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut cubes = HashSet::new();

    let coords = parse_uints(input);

    for chunk in coords.chunks(3) {
        cubes.insert((chunk[0] as i32, chunk[1] as i32, chunk[2] as i32));
    }

    PuzzleInput { cubes }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
