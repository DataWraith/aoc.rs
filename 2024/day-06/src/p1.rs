use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let coord = guard_starting_position(&input.grid);

    let mut visited = HashSet::new();
    visited.insert((coord, Direction::Up));

    let mut state = GuardState {
        coordinate: coord,
        direction: Direction::Up,
        steps: 0,
        visited,
    };

    while let Some(next_state) = state.next_state(&input.grid) {
        state = next_state;
    }

    let mut v = state.visited.iter().map(|(c, _)| c).collect::<Vec<_>>();
    v.sort();
    v.dedup();

    v.len().to_string()
}

#[derive(Debug)]
pub struct GuardState {
    pub coordinate: Coordinate,
    pub direction: Direction,
    pub steps: usize,
    pub visited: HashSet<(Coordinate, Direction)>,
}

impl GuardState {
    pub fn next_state(&self, grid: &Grid2D<char>) -> Option<GuardState> {
        let next_coord = self.coordinate + self.direction.into();

        if let Some(c) = grid.get(next_coord) {
            if *c == '#' {
                let new_direction = self.direction.turn_right_90();

                return Some(GuardState {
                    coordinate: self.coordinate,
                    direction: new_direction,
                    steps: self.steps + 1,
                    visited: self.visited.clone(),
                });
            }

            let mut visited = self.visited.clone();
            visited.insert((next_coord, self.direction));

            return Some(GuardState {
                coordinate: next_coord,
                direction: self.direction,
                steps: self.steps + 1,
                visited,
            });
        }

        None
    }
}

pub fn guard_starting_position(grid: &Grid2D<char>) -> Coordinate {
    grid.iter()
        .find(|(c, x)| **x == '^')
        .map(|(c, x)| c)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
