use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap},
};

use petgraph::graph::{DiGraph, NodeIndex};
use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    return race(input).to_string();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub position: Coordinate,
    pub direction: Direction,
    pub heuristic: usize,
    pub straight: usize,
    pub turn: usize,
}

impl State {
    fn score(&self) -> usize {
        self.straight + self.turn * 1000
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.score() + other.heuristic).cmp(&(self.score() + self.heuristic))
    }
}

pub fn follow_path(
    cur: Coordinate,
    dir: Direction,
    grid: &Grid2D<char>,
    junctions: &HashSet<Coordinate>,
) -> (Coordinate, Direction, usize) {
    let mut cur = cur;
    let mut len = 1;

    let (cur, dir, len) = loop {
        let next = cur + dir;

        if junctions.contains(&next) {
            break (next, dir, len);
        }

        if grid.get(next) == Some(&'E') {
            break (next, dir, len);
        }

        if grid.get(next) == Some(&'#') {
            break (cur, dir, len - 1);
        }

        len += 1;
        cur = next;
    };

    (cur, dir, len)
}

pub fn race(input: &PuzzleInput) -> usize {
    let junctions = junctions(&input.maze);

    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut q = BinaryHeap::new();
    let mut visited = BTreeSet::new();

    q.push(State {
        position: start,
        direction: Direction::Right,
        straight: 0,
        turn: 0,
        heuristic: start.manhattan_distance(end) as usize,
    });

    let mut best = usize::MAX;

    while let Some(state) = q.pop() {
        if visited.contains(&(state.position, state.direction)) {
            continue;
        }

        visited.insert((state.position, state.direction));

        if visited.len() % 1000 == 0 {
            println!("{}", visited.len());
        }

        if false || state.position == end {
            let mut grid = input.maze.clone();
            let c = match state.direction {
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Up => '^',
                _ => unreachable!(),
            };
            grid.set(state.position, c);
            println!("{}\n", grid);
            dbg!(&state);
            //std::thread::sleep(std::time::Duration::from_millis(500));
            if state.position == end {
                best = state.score().min(best);
            }
        }

        let (next, dir, len) =
            follow_path(state.position, state.direction, &input.maze, &junctions);

        let fwd_free = state.position != next;

        if fwd_free {
            let n = state.position.neighbor(state.direction);
            let mut fwd = State {
                position: next,
                straight: state.straight + len,
                heuristic: n.manhattan_distance(end) as usize,
                ..state.clone()
            };

            q.push(fwd);
        }

        if junctions.contains(&state.position) || !fwd_free {
            let mut turn_left = State {
                direction: state.direction.turn_left_90(),
                turn: state.turn + 1,
                ..state.clone()
            };

            let n = turn_left.position.neighbor(turn_left.direction);
            if input.maze.get(n) == Some(&'.') || input.maze.get(n) == Some(&'E') {
                q.push(turn_left);
            }

            let mut turn_right = State {
                direction: state.direction.turn_right_90(),
                turn: state.turn + 1,
                ..state
            };

            let n = turn_right.position.neighbor(turn_right.direction);
            if input.maze.get(n) == Some(&'.') || input.maze.get(n) == Some(&'E') {
                q.push(turn_right);
            }
        }
    }

    return best;
}

pub fn junctions(maze: &Grid2D<char>) -> HashSet<Coordinate> {
    let mut junctions = HashSet::new();

    junctions.insert(maze.iter().find(|(_, &c)| c == 'S').unwrap().0);

    for (coord, &c) in maze.iter() {
        let mut count = 0;
        for neighbors in coord.neighbors() {
            if maze.get(neighbors) == Some(&'.') {
                count += 1;
            }
        }

        if count > 2 {
            junctions.insert(coord);
        }
    }

    junctions
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
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
