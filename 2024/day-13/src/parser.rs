use glam::U64Vec2;
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub games: Vec<ClawGame>,
    pub offset: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ClawGame {
    pub offset_a: U64Vec2,
    pub offset_b: U64Vec2,
    pub prize: U64Vec2,
}

pub fn part1(input: &str) -> PuzzleInput {
    let mut result = Vec::new();

    let games = input.split("\n\n");

    for g in games.into_iter() {
        let coordinates = parse_uints(g);

        let offset_a = U64Vec2::new(coordinates[0], coordinates[1]);
        let offset_b = U64Vec2::new(coordinates[2], coordinates[3]);
        let prize = U64Vec2::new(coordinates[4], coordinates[5]);

        result.push(ClawGame {
            offset_a,
            offset_b,
            prize,
        });
    }

    PuzzleInput {
        games: result,
        offset: 0,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
