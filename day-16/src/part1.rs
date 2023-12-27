use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    solve(
        input,
        Beam {
            position: Coordinate::new(-1, 0),
            direction: Direction::Right,
        },
    )
    .to_string()
}

pub fn solve(input: &PuzzleInput, start: Beam) -> usize {
    let mut energized = HashSet::new();

    let mut brfs = BrFS::new(start);

    while let Some(beam) = brfs.next(|beam| traverse(&input.grid, beam)) {
        energized.insert(beam.position);
    }

    // Subtract the starting tile outside of the grid
    energized.len() - 1
}

pub fn traverse(grid: &Grid2D<char>, beam: &Beam) -> ArrayVec<[Beam; 2]> {
    let mut result: ArrayVec<[Beam; 2]> = ArrayVec::new();

    let next_coord = beam.position + beam.direction.into();

    match grid.get(next_coord) {
        None => return result,

        Some('.') => result.push(Beam {
            position: next_coord,
            direction: beam.direction,
        }),

        Some('/') => match beam.direction {
            Direction::Up | Direction::Down => result.push(Beam {
                position: next_coord,
                direction: beam.direction.turn_right(),
            }),

            Direction::Left | Direction::Right => result.push(Beam {
                position: next_coord,
                direction: beam.direction.turn_left(),
            }),
        },

        Some('\\') => match beam.direction {
            Direction::Up | Direction::Down => result.push(Beam {
                position: next_coord,
                direction: beam.direction.turn_left(),
            }),

            Direction::Right | Direction::Left => result.push(Beam {
                position: next_coord,
                direction: beam.direction.turn_right(),
            }),
        },

        Some('-') => match beam.direction {
            Direction::Right | Direction::Left => result.push(Beam {
                position: next_coord,
                direction: beam.direction,
            }),

            Direction::Up | Direction::Down => {
                result.push(Beam {
                    position: next_coord,
                    direction: beam.direction.turn_left(),
                });
                result.push(Beam {
                    position: next_coord,
                    direction: beam.direction.turn_right(),
                });
            }
        },

        Some('|') => match beam.direction {
            Direction::Up | Direction::Down => result.push(Beam {
                position: next_coord,
                direction: beam.direction,
            }),

            Direction::Right | Direction::Left => {
                result.push(Beam {
                    position: next_coord,
                    direction: beam.direction.turn_left(),
                });
                result.push(Beam {
                    position: next_coord,
                    direction: beam.direction.turn_right(),
                });
            }
        },

        _ => unreachable!("Invalid tile at {:?}", next_coord),
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "#};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "46");
    }
}
