use utility_belt::prelude::*;

use crate::parser::*;

#[derive(Clone, Debug)]
pub struct State {
    pub position: Coordinate,
    pub direction: Direction,
}

impl State {
    pub fn new(input: &PuzzleInput) -> Self {
        let coord = input.costs.iter().find(|(_coord, c)| **c == 1).unwrap().0;
        Self {
            position: coord,
            direction: Direction::Right,
        }
    }

    pub fn step(&mut self, input: &PuzzleInput, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::TurnLeft => State {
                direction: self.direction.turn_left_90(),
                ..self.clone()
            },
            Instruction::TurnRight => State {
                direction: self.direction.turn_right_90(),
                ..self.clone()
            },
            Instruction::Move(n) => {
                let mut next_position = self.position;
                let mut cost = 0;

                for _ in 0.. {
                    next_position = next_position.neighbor(self.direction);
                    let next_cost = input.costs.get_wrap(next_position);

                    if *next_cost == 255 {
                        while *input.costs.get_wrap(next_position) != 1 {
                            next_position = next_position.neighbor(self.direction.opposite());
                        }

                        break;
                    }

                    cost += next_cost;

                    if cost == *n {
                        break;
                    }
                }

                let next = Coordinate::new(
                    next_position.x.rem_euclid(input.costs.width() as i32),
                    next_position.y.rem_euclid(input.costs.height() as i32),
                );

                State {
                    position: next,
                    direction: self.direction,
                }
            }
        }
    }
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut state = State::new(input);

    for instruction in input.instructions.iter() {
        state = state.step(input, instruction);
    }

    let final_column = state.position.x + 1;
    let final_row = state.position.y + 1;
    let final_facing = match state.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => unreachable!(),
    };

    let password = final_row * 1000 + final_column * 4 + final_facing;

    password.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "6032");
    }
}
