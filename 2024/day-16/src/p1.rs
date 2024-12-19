use std::{cmp::Ordering, collections::BinaryHeap, rc::Rc};

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    search(input).0.to_string()
}

// A waypoint is a position in the maze that we have visited. This struct stores
// the current position and a pointer to the previous waypoint, which allows us
// to reconstruct the path. The Waypoints form a tree structure, so we only pay
// for each path prefix once.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Waypoints {
    current: Coordinate,
    previous: Option<Rc<Waypoints>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub position: Coordinate,
    pub direction: Direction,
    pub heuristic: usize,
    pub straight_steps: usize,
    pub number_of_turns: usize,
    pub waypoints: Rc<Waypoints>,
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

// Runs the A* algorithm to find all shortest paths from start to end.
pub fn search(input: &PuzzleInput) -> (usize, Grid2D<bool>) {
    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut best_score = usize::MAX;
    let mut best_waypoints = Vec::new();

    let mut pq = BinaryHeap::new();
    let mut lowest_cost = HashMap::new();

    // Create a waypoints object for the start position.
    let start_wp = Rc::new(Waypoints {
        current: start,
        previous: None,
    });

    // Starting states
    pq.push(State {
        position: start,
        direction: Direction::Right,
        straight_steps: 0,
        number_of_turns: 0,
        heuristic: heuristic(start, Direction::Right, end),
        waypoints: start_wp.clone(),
    });

    pq.push(State {
        position: start,
        direction: Direction::Up,
        straight_steps: 0,
        number_of_turns: 1,
        heuristic: heuristic(start, Direction::Right, end),
        waypoints: start_wp,
    });

    while let Some(state) = pq.pop() {
        if state.position == end {
            // Non-shortest path found? That means that we have found all shortest
            // paths, so we can exit the search.
            if state.score() > best_score {
                break;
            };

            // Otherwise, we have found a new shortest path.
            best_score = state.score();
            best_waypoints.push(state.waypoints.clone());

            continue;
        }

        for next_state in successors(input, end, state) {
            let prev_cost = lowest_cost
                .get(&(next_state.position, next_state.direction))
                .unwrap_or(&usize::MAX);

            if next_state.score() > *prev_cost {
                continue;
            }

            if next_state.score() < *prev_cost {
                lowest_cost.insert(
                    (next_state.position, next_state.direction),
                    next_state.score(),
                );
            }

            pq.push(next_state);
        }
    }

    (best_score, coverage(input, best_waypoints))
}

// Returns all possible next states from the current state.
fn successors(input: &PuzzleInput, end: Coordinate, state: State) -> Vec<State> {
    let mut steps = 0;
    let mut next_states = Vec::new();

    for pos in std::iter::successors(Some(state.position + state.direction), |pos| {
        Some(pos.neighbor(state.direction))
    }) {
        steps += 1;

        if input.maze.get(pos) == Some(&'#') {
            break;
        }

        if input.maze.get(pos) == Some(&'E') {
            let waypoints = Rc::new(Waypoints {
                current: pos,
                previous: Some(state.waypoints.clone()),
            });

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
                let waypoints = Rc::new(Waypoints {
                    current: pos,
                    previous: Some(state.waypoints.clone()),
                });

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
    }

    next_states
}

// Returns a grid of all the positions that the best paths have covered.
// We do this by iterating over all the waypoints and marking the positions
// on straight lines between the waypoints.
fn coverage(input: &PuzzleInput, best_waypoints: Vec<Rc<Waypoints>>) -> Grid2D<bool> {
    let mut seen = input.maze.map(|_| false);

    for wp in best_waypoints {
        seen[wp.current] = true;

        let mut current = wp;

        while let Some(prev) = current.previous.clone() {
            let a = current.current;
            let b = prev.current;
            let dir = a.towards(b);

            let mut pos = a;
            while pos != b {
                pos = pos.neighbor(dir);
                seen[pos] = true;
            }

            current = prev;
        }
    }
    seen
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
