use crate::{
    p1::{guard_starting_position, GuardState},
    structs::*,
};

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let coord = guard_starting_position(&input.grid);

    let mut state = GuardState {
        coordinate: coord,
        direction: Direction::Up,
    };

    let mut visited = Vec::new();

    while let Some(next_state) = state.next_state(&input.grid) {
        visited.push((next_state.coordinate, next_state.direction));
        state = next_state;
    }

    visited.push((state.coordinate, state.direction));

    let mut loops_found = HashSet::new();
    let mut obstacles = HashSet::new();

    for (pos, dir) in visited.iter() {
        let obstacle = *pos + dir.clone().into();

        if obstacle == coord {
            continue;
        }

        if input.grid.get(obstacle).is_none() {
            continue;
        }

        if input.grid.get(obstacle).unwrap() == &'#' {
            continue;
        }

        obstacles.insert(obstacle);
    }

    for obstacle in obstacles.iter() {
        let mut g2 = input.grid.clone();
        g2.set(*obstacle, '#');

        let mut state = GuardState {
            coordinate: coord,
            direction: Direction::Up,
        };

        let mut visited = HashSet::new();

        while let Some(next_state) = state.next_state(&g2) {
            state = next_state;

            if !visited.insert((state.coordinate, state.direction)) {
                break;
            }
        }

        if !g2.get(state.coordinate + state.direction.into()).is_none() {
            loops_found.insert(obstacle);
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
