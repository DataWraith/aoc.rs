use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct Robot {
    pub position: Coordinate,
    pub velocity: Coordinate,
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub robots: Vec<Robot>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let ints = parse_ints(input);
    let mut robots = Vec::new();

    for robot in ints.chunks(4) {
        let position = Coordinate::new(robot[0] as i32, robot[1] as i32);
        let velocity = Coordinate::new(robot[2] as i32, robot[3] as i32);
        robots.push(Robot { position, velocity });
    }

    PuzzleInput { robots }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
