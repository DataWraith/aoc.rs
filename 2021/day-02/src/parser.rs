use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let (command, amount) = s.split_once(' ').unwrap();

        match command {
            "forward" => Self::Forward(amount.parse().unwrap()),
            "up" => Self::Up(amount.parse().unwrap()),
            "down" => Self::Down(amount.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub commands: Vec<Command>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    PuzzleInput {
        commands: input.lines().map(|line| Command::from(line)).collect(),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
