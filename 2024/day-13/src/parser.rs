use glam::U64Vec2;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub games: Vec<ClawGame>,
    pub part2: bool,
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
        let lines = g.lines().collect::<Vec<_>>();

        let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();

        let mut offset_a = U64Vec2::ZERO;
        let mut offset_b = U64Vec2::ZERO;
        let mut prize = U64Vec2::ZERO;

        for (_, [x, y]) in re.captures_iter(lines[0]).map(|c| c.extract()) {
            offset_a = U64Vec2::new(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap());
        }

        for (_, [x, y]) in re.captures_iter(lines[1]).map(|c| c.extract()) {
            offset_b = U64Vec2::new(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap());
        }

        let re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

        for (_, [x, y]) in re.captures_iter(lines[2]).map(|c| c.extract()) {
            prize = U64Vec2::new(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap());
        }

        result.push(ClawGame {
            offset_a,
            offset_b,
            prize,
        });
    }

    PuzzleInput {
        games: result,
        part2: false,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
