use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub costs: Grid2D<u32>,
    pub instructions: Vec<Instruction>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    PuzzleInput {
        costs: parse_map(map),
        instructions: parse_instructions(instructions),
    }
}

fn parse_map(input: &str) -> Grid2D<u32> {
    let max_length = input.lines().map(|line| line.len()).max().unwrap();
    let mut grid: Grid2D<u32> = Grid2D::new(max_length, input.lines().count(), 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coordinate::new(x as i32, y as i32);

            grid.set(
                coord,
                match c {
                    '.' => 1,
                    '#' => u32::MAX,
                    _ => 0,
                },
            );
        }
    }

    grid
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let uints = parse_ints(input);
    let capitals = parse_capitals(input);

    let mut instructions = Vec::new();

    for (i, c) in uints.iter().zip(capitals.iter()) {
        instructions.push(Instruction::Move(*i as u32));

        if *c == "L" {
            instructions.push(Instruction::TurnLeft);
        } else if *c == "R" {
            instructions.push(Instruction::TurnRight);
        } else {
            panic!("Invalid instruction: {}", c);
        }
    }

    instructions.push(Instruction::Move(*uints.last().unwrap() as u32));

    instructions
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
