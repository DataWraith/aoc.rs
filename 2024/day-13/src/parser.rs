use regex::Regex;
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub games: Vec<ClawGame>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ClawGame {
    pub offset_a: Coordinate,
    pub offset_b: Coordinate,
    pub prize: Coordinate,
}

pub fn part1(input: &str) -> PuzzleInput {
    let mut result = Vec::new();

    let games = input.split("\n\n");

    for g in games.into_iter() {
        let lines = g.lines().collect::<Vec<_>>();

        let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();

        let mut offset_a = Coordinate::new(0, 0);
        let mut offset_b = Coordinate::new(0, 0);
        let mut prize = Coordinate::new(0, 0);

        for (_, [x, y]) in re.captures_iter(lines[0]).map(|c| c.extract()) {
            offset_a = Coordinate::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        }

        for (_, [x, y]) in re.captures_iter(lines[1]).map(|c| c.extract()) {
            offset_b = Coordinate::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        }

        let re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

        for (_, [x, y]) in re.captures_iter(lines[2]).map(|c| c.extract()) {
            prize = Coordinate::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        }

        result.push(ClawGame {
            offset_a,
            offset_b,
            prize,
        });
    }

    PuzzleInput { games: result }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
