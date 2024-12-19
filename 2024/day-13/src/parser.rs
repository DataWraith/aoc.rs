use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub games: Vec<ClawGame>,
    pub offset: i64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ClawGame {
    pub offset_a: I64Vec2,
    pub offset_b: I64Vec2,
    pub prize: I64Vec2,
}

pub fn part1(input: &str) -> PuzzleInput {
    let mut result = Vec::new();

    let games = input.split("\n\n");

    for g in games.into_iter() {
        let coordinates = parse_uints(g);

        let offset_a = I64Vec2::new(coordinates[0] as i64, coordinates[1] as i64);
        let offset_b = I64Vec2::new(coordinates[2] as i64, coordinates[3] as i64);
        let prize = I64Vec2::new(coordinates[4] as i64, coordinates[5] as i64);

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
