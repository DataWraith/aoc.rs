use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    search(input)[0].score().to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

// Follows a given straight path until it either hits a junction, the end, or a wall.
pub fn follow_path(
    cur: Coordinate,
    dir: Direction,
    grid: &Grid2D<char>,
    junctions: &Grid2D<bool>,
) -> (Coordinate, usize) {
    let mut cur = cur;
    let mut len = 1;

    loop {
        let next = cur + dir;

        if junctions.get(next) == Some(&true) {
            return (next, len);
        }

        if grid.get(next) == Some(&'#') {
            return (cur, len - 1);
        }

        if grid.get(next) == Some(&'E') {
            return (next, len);
        }

        len += 1;
        cur = next;
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
pub fn search(input: &PuzzleInput) -> Vec<State> {
    let junctions = junctions(&input.maze);

    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut q = BinaryHeap::new();
    let mut visited = BTreeMap::new();

    // Starting states
    q.push(State {
        position: start,
        direction: Direction::Right,
        straight_steps: 0,
        number_of_turns: 0,
        waypoints: vec![start],
        heuristic: heuristic(start, Direction::Right, end),
    });

    let mut best: Vec<State> = vec![];

    while let Some(state) = q.pop() {
        // Skip if we've already seen this state with a better score
        if let Some(prev_g) = visited.get(&(state.position, state.direction)) {
            if state.score() > *prev_g {
                continue;
            }
        }

        // Mark this state as visited
        visited.insert((state.position, state.direction), state.score());

        // If we've reached the end, add this state to the best list. We are guaranteed
        // to only add shortest paths to the best list because we are using a priority queue
        // and an admissible heuristic. Also, we break out of the loop below if we've found
        // all shortest paths, so we can't reach this point if we've already found all
        // shortest paths.
        if state.position == end {
            if best.is_empty() || state.score() == best[0].score() {
                best.push(state.clone());
            }

            continue;
        }

        // Follow the current corridor until we hit a junction, the end, or a wall.
        let (next, len) = follow_path(state.position, state.direction, &input.maze, &junctions);

        // Did we hit a wall?
        let hit_a_wall = len == 0;

        // If we didn't hit a wall, this is a crossing, and we can proceed forwards.
        if !hit_a_wall {
            // Update the list of waypoints
            let mut waypoints = state.waypoints.clone();
            waypoints.push(next);

            // Add the new state to the queue
            let fwd = State {
                position: next,
                straight_steps: state.straight_steps + len,
                heuristic: heuristic(next, state.direction, end),
                waypoints,
                ..state.clone()
            };

            q.push(fwd);
        }

        // If we are at a junction or corner, we can try to turn left or right.
        if junctions.get(state.position) == Some(&true) || hit_a_wall {
            let left_dir = state.direction.turn_left_90();
            let right_dir = state.direction.turn_right_90();

            for dir in [left_dir, right_dir] {
                let coord = state.position.neighbor(dir);

                if input.maze.get(coord) == Some(&'.') || input.maze.get(coord) == Some(&'E') {
                    let turn = State {
                        direction: dir,
                        number_of_turns: state.number_of_turns + 1,
                        ..state.clone()
                    };

                    q.push(turn);
                }
            }
        }

        // If the current state is worse than the best state we've found, we can stop early,
        // because we are guaranteed to have found all shortest paths (due to the properties
        // of A* and the admissable heuristic).
        if !best.is_empty() && state.score() > best[0].score() {
            break;
        }
    }

    best
}

pub fn junctions(maze: &Grid2D<char>) -> Grid2D<bool> {
    let mut junctions = maze.map(|_| false);

    junctions.set(maze.iter().find(|(_, &c)| c == 'S').unwrap().0, true);

    for (coord, &c) in maze.iter() {
        if c != '.' {
            continue;
        }

        let mut count = 0;
        for neighbors in coord.neighbors() {
            if maze.get(neighbors) == Some(&'.') {
                count += 1;
            }
        }

        if count > 2 {
            junctions.set(coord, true);
        }
    }

    junctions
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
