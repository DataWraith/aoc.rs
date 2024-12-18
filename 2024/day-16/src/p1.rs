use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

use std::iter::successors;
use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    search(input).0.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub position: Coordinate,
    pub direction: Direction,
    pub heuristic: usize,
    pub straight_steps: usize,
    pub number_of_turns: usize,
    pub waypoints: Vec<Coordinate>,
}

impl State {
    pub fn score(&self) -> usize {
        self.straight_steps + self.number_of_turns * 1000
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Ordering function for the priority queue -- we want to prioritize states with
// lower scores + heuristic values to get the A* algorithm. Since BinaryHeap is
// a max-heap, we invert the comparison.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.score() + other.heuristic).cmp(&(self.score() + self.heuristic))
    }
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

// Runs the A* algorithm to find the shortest path from start to end.
pub fn search(input: &PuzzleInput) -> (usize, Vec<Vec<Coordinate>>) {
    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut best_score = usize::MAX;
    let mut best_waypoints = Vec::new();

    let mut q = BinaryHeap::new();
    let mut lowest_cost = BTreeMap::new();

    // Starting states
    q.push(State {
        position: start,
        direction: Direction::Right,
        straight_steps: 0,
        number_of_turns: 0,
        heuristic: heuristic(start, Direction::Right, end),
        waypoints: vec![start],
    });

    q.push(State {
        position: start,
        direction: Direction::Up,
        straight_steps: 0,
        number_of_turns: 1,
        heuristic: heuristic(start, Direction::Right, end),
        waypoints: vec![start],
    });

    while let Some(state) = q.pop() {
        // Skip if we've already seen this state with a better score
        if let Some(prev_g) = lowest_cost.get(&(state.position, state.direction)) {
            if state.score() > *prev_g {
                continue;
            }
        }

        // If we've reached the end, add this state to the best list. We are guaranteed
        // to only add shortest paths to the best list because we are using a priority queue
        // and an admissible heuristic. Also, we break out of the loop below if we've found
        // all shortest paths, so we can't reach this point if we've already found all
        // shortest paths.
        if state.position == end {
            if state.score() > best_score {
                break;
            };

            best_score = state.score();
            best_waypoints.push(state.waypoints.clone());

            continue;
        }

        let mut steps = 0;
        for pos in successors(Some(state.position + state.direction), |pos| {
            Some(pos.neighbor(state.direction))
        }) {
            steps += 1;

            if input.maze.get(pos) == Some(&'#') {
                break;
            }

            let mut next_states = Vec::new();

            if input.maze.get(pos) == Some(&'E') {
                let mut waypoints = state.waypoints.clone();
                waypoints.push(pos);

                let next_state = State {
                    position: pos,
                    direction: state.direction,
                    straight_steps: state.straight_steps + steps,
                    number_of_turns: state.number_of_turns,
                    heuristic: 0,
                    waypoints,
                };

                next_states.push(next_state);
            }

            for dir in [Direction::Left, Direction::Right] {
                let next_dir = if dir == Direction::Left {
                    state.direction.turn_left_90()
                } else {
                    state.direction.turn_right_90()
                };

                let next_pos = pos.neighbor(next_dir);

                if input.maze.get(next_pos) == Some(&'.')
                    || input.maze.get(next_pos) == Some(&'E')
                    || input.maze.get(next_pos) == Some(&'S')
                {
                    let mut waypoints = state.waypoints.clone();
                    waypoints.push(pos);

                    let next_state = State {
                        position: pos,
                        direction: next_dir,
                        straight_steps: state.straight_steps + steps,
                        number_of_turns: state.number_of_turns + 1,
                        heuristic: heuristic(pos, next_dir, end),
                        waypoints,
                    };

                    next_states.push(next_state);
                }
            }

            for next_state in next_states {
                let lowest = lowest_cost
                    .get(&(next_state.position, next_state.direction))
                    .unwrap_or(&usize::MAX);

                if next_state.score() > *lowest {
                    continue;
                }

                if next_state.score() < *lowest {
                    lowest_cost.insert(
                        (next_state.position, next_state.direction),
                        next_state.score(),
                    );
                }

                q.push(next_state);
            }
        }
    }

    (best_score, best_waypoints)
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
