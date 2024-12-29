use utility_belt::prelude::*;

use crate::parser::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    pub fn step(
        &mut self,
        input: &PuzzleInput,
        connections: &HashMap<Self, Self>,
        instruction: &Instruction,
    ) -> Self {
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
                let mut next = self.clone();
                let mut cost = 0;

                for _ in 0.. {
                    if let Some(connected) = connections.get(&next) {
                        if connected.position == next.position {
                            break;
                        }

                        next = connected.clone();
                        cost += 1;
                    } else {
                        next = State {
                            position: next.position.neighbor(next.direction),
                            direction: next.direction,
                        };

                        let next_cost = input.costs.get_wrap(next.position);

                        if *next_cost == u32::MAX {
                            while *input.costs.get_wrap(next.position) != 1 {
                                next.position = next.position.neighbor(next.direction.opposite());
                            }

                            break;
                        }

                        cost += next_cost;
                    }

                    if cost == *n {
                        break;
                    }
                }

                let wrap_pos = Coordinate::new(
                    next.position.x.rem_euclid(input.costs.width() as i32),
                    next.position.y.rem_euclid(input.costs.height() as i32),
                );

                State {
                    position: wrap_pos,
                    ..next
                }
            }
        }
    }
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut state = State::new(input);

    for instruction in input.instructions.iter() {
        state = state.step(input, &HashMap::new(), instruction);
    }

    compute_password(state)
}

pub fn compute_password(state: State) -> String {
    // One-based indexing
    let final_column = state.position.x + 1;
    let final_row = state.position.y + 1;

    let final_facing = match state.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => unreachable!(),
    };

    (final_row * 1000 + final_column * 4 + final_facing).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
