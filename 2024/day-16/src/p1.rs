use pathfinding::prelude::astar_bag;
use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    search(input).0.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub position: Coordinate,
    pub direction: Direction,
}

// Heuristic function for the A* algorithm.
//
// We need to take at least manhattan_distance(end) straight steps, but since the end
// is in the top-right corner, we need to turn at least once if we are currently looking
// leftwards or downwards.
fn heuristic(cur: Coordinate, dir: Direction, end: Coordinate) -> usize {
    let mut h = cur.manhattan_distance(end) as usize;

    if dir == Direction::Left || dir == Direction::Down {
        h += 1000;
    }

    h
}

fn update_coverage(coverage: &mut Grid2D<bool>, path: Vec<State>) {
    path.windows(2).for_each(|w| {
        let a = w[0].position;
        let b = w[1].position;
        let dir = a.towards(b);

        coverage[a] = true;

        let mut pos = a;
        while pos != b {
            pos = pos.neighbor(dir);
            coverage[pos] = true;
        }
    });
}

pub fn search(input: &PuzzleInput) -> (usize, Grid2D<bool>) {
    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut best_score = usize::MAX;
    let mut coverage = input.maze.map(|_| false);

    let start_state = State {
        position: start,
        direction: Direction::Right,
    };

    if let Some((solution, score)) = astar_bag(
        &start_state,
        |state| successors(input, start, end, state),
        |state| heuristic(state.position, state.direction, end),
        |state| state.position == end,
    ) {
        best_score = score;

        for states in solution {
            update_coverage(&mut coverage, states);
        }
    }

    (best_score, coverage)
}

// Returns all possible next states from the current state.
fn successors(
    input: &PuzzleInput,
    start: Coordinate,
    end: Coordinate,
    state: &State,
) -> Vec<(State, usize)> {
    let mut next_states = Vec::new();

    if state.position == start && state.direction == Direction::Right {
        next_states.push((
            State {
                position: start,
                direction: Direction::Up,
            },
            1000,
        ));
    }

    let mut steps = 0;
    for pos in std::iter::successors(Some(state.position + state.direction), |pos| {
        Some(pos.neighbor(state.direction))
    }) {
        steps += 1;

        // We can't move into walls
        if input.maze.get(pos) == Some(&'#') {
            break;
        }

        // Otherwise, hitting the goal is the only time we stop when moving forward
        if pos == end {
            let next_state = State {
                position: pos,
                direction: state.direction,
            };

            next_states.push((next_state, steps));
        }

        for dir in [Direction::Left, Direction::Right] {
            let next_dir = if dir == Direction::Left {
                state.direction.turn_left_90()
            } else {
                state.direction.turn_right_90()
            };

            let next_pos = pos.neighbor(next_dir);

            if input.maze.get(next_pos) == Some(&'.') {
                let next_state = State {
                    position: pos,
                    direction: next_dir,
                };

                next_states.push((next_state, steps + 1000));
            }
        }
    }

    next_states
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "7036");
    }
}
