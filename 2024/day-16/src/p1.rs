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

impl State {
    // Heuristic function for the A* algorithm.
    //
    // We need to take at least manhattan_distance(end) straight steps, but since the end
    // is in the top-right corner, we need to turn at least once if we are currently looking
    // leftwards or downwards.
    pub fn heuristic(&self, end: Coordinate) -> usize {
        let mut h = self.position.manhattan_distance(end) as usize;

        if self.direction == Direction::Left || self.direction == Direction::Down {
            h += 1000;
        }

        h
    }
}

// Connect the waypoints on the path with straight lines in order to figure out
// which squares were on the shortest path.
fn update_coverage(coverage: &mut BoolGrid2D, path: Vec<State>) {
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

// Find the shortest paths from S to E and compute the coverage of the maze.
pub fn search(input: &PuzzleInput) -> (usize, BoolGrid2D) {
    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut best_score = usize::MAX;
    let mut coverage = input.maze.map(|_| false).into();

    let start_state = State {
        position: start,
        direction: Direction::Right,
    };

    if let Some((solution, score)) = astar_bag(
        &start_state,
        |state| successors(input, start, end, state),
        |state| state.heuristic(end),
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

        // But we will try to turn left or right at every step.
        for dir in [Direction::Left, Direction::Right] {
            let next_dir = if dir == Direction::Left {
                state.direction.turn_left_90()
            } else {
                state.direction.turn_right_90()
            };

            let next_pos = pos.neighbor(next_dir);

            if input.maze.get(next_pos) == Some(&'.') {
                // Note that we can't move to next_pos directly, because then
                // our waypoints wouldn't be connected by straight lines anymore,
                // which would make the coverage calculation fail.
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
