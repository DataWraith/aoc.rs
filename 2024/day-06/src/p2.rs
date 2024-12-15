use crate::{
    p1::{guard_starting_position, GuardState},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let coord = guard_starting_position(&input.grid);

    let mut state = GuardState {
        coordinate: coord,
        direction: Direction::Up,
    };

    let mut visited = Vec::new();
    visited.push((state.coordinate, state.direction));

    while let Some(next_state) = state.next_state(&input.grid) {
        visited.push((next_state.coordinate, next_state.direction));
        state = next_state;
    }

    let mut loops_found = HashSet::new();
    let mut obstacles = HashSet::new();

    // Theoretically, we could extract the first time we land on a particular coordinate,
    // and use the previuos step as starting point to find the loop. But this is fast enough.

    for (pos, dir) in visited.iter() {
        let obstacle = *pos + (*dir);

        // Can't place an obstacle on the starting position
        if obstacle == coord {
            continue;
        }

        // Can't place an obstacle outside the lab
        if input.grid.get(obstacle).is_none() {
            continue;
        }

        // Can't place an obstacle on an obstacle
        if input.grid.get(obstacle).unwrap() == &'#' {
            continue;
        }

        obstacles.insert(obstacle);
    }

    let mut g2 = input.grid.clone();

    for obstacle in obstacles.iter() {
        g2.set(*obstacle, '#');

        let mut state = GuardState {
            coordinate: coord,
            direction: Direction::Up,
        };

        let mut visited = g2.map(|_| DirectionSet::empty());

        let mut initial_dirset = DirectionSet::empty();
        initial_dirset.insert(state.direction);
        visited.set(state.coordinate, initial_dirset);

        while let Some(next_state) = state.next_state(&g2) {
            state = next_state;

            if let Some(dirs) = visited.get_mut(state.coordinate) {
                if dirs.contains(state.direction) {
                    loops_found.insert(obstacle);
                    break;
                }

                dirs.insert(state.direction);
            }
        }

        g2.set(*obstacle, '.');
    }

    loops_found.len().to_string()
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "6");
    }
}
