use crate::{
    p1::{guard_starting_position, GuardState},
    structs::*,
};

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
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

    let mut loops_found = HashSet::new();

    for obstacle in state.visited {
        if obstacle.0 == coord {
            continue;
        }

        let mut guard_loop = HashSet::new();

        let mut g2 = input.grid.clone();
        g2.set(obstacle.0, '#');

        let mut visited = HashSet::new();
        visited.insert((coord, Direction::Up));

        let mut state = GuardState {
            coordinate: coord,
            direction: Direction::Up,
            steps: 0,
            visited,
        };

        while let Some(next_state) = state.next_state(&g2) {
            if !guard_loop.insert((next_state.coordinate, next_state.direction)) {
                loops_found.insert(obstacle.0);
                break;
            }

            state = next_state;
        }
    }

    loops_found.len().to_string()
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "6");
    }
}
