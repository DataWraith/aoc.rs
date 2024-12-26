use utility_belt::prelude::*;

use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    let coord = guard_starting_position(&input.grid);

    let mut state = GuardState {
        coordinate: coord,
        direction: Direction::Up,
    };

    let mut coords_visited = 1;
    let mut visited = input.grid.map(|_x| false);
    visited.set(state.coordinate, true);

    while let Some(next_state) = state.next_state(&input.grid) {
        if visited.get(next_state.coordinate) == Some(&false) {
            coords_visited += 1;
            visited.set(next_state.coordinate, true);
        }

        state = next_state;
    }

    coords_visited.to_string()
}

#[derive(Debug)]
pub struct GuardState {
    pub coordinate: Coordinate,
    pub direction: Direction,
}

impl GuardState {
    pub fn next_state(&self, grid: &Grid2D<char>) -> Option<GuardState> {
        let next_coord = self.coordinate + self.direction;

        if let Some(c) = grid.get(next_coord) {
            if *c == '#' {
                let new_direction = self.direction.turn_right_90();

                return Some(GuardState {
                    coordinate: self.coordinate,
                    direction: new_direction,
                });
            }

            return Some(GuardState {
                coordinate: next_coord,
                direction: self.direction,
            });
        }

        None
    }
}

pub fn guard_starting_position(grid: &Grid2D<char>) -> Coordinate {
    grid.iter()
        .find(|(_c, x)| **x == '^')
        .map(|(c, _x)| c)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(part1(&input), "41");
    }
}
