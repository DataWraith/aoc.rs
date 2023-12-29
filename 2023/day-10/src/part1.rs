use std::collections::VecDeque;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let start = input.grid.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let start_pipes = find_start_pipes(start, input);

    follow_pipes(input, start, start_pipes).to_string()
}

pub fn find_start_pipes(start: Coordinate, input: &PuzzleInput) -> [Coordinate; 2] {
    let mut start_pipes = [Coordinate::new(-1, -1); 2];

    let mut idx = 0;

    for d in Direction::all() {
        let n = start.neighbor(d);
        if let Some(c) = input.grid.get(n) {
            if adjacent(*c).contains(d.opposite()) {
                start_pipes[idx] = n;
                idx += 1;
            }
        }
    }
    start_pipes
}

fn follow_pipes(input: &PuzzleInput, start: Coordinate, start_pipes: [Coordinate; 2]) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::default();
    let mut d = 0;

    queue.push_back((start_pipes[0], start, 1));
    queue.push_back((start_pipes[1], start, 1));

    while let Some((cur, prev, dist)) = queue.pop_front() {
        if !seen.insert(cur) {
            d = d.max(dist - 1);
            continue;
        }

        let c = input.grid[cur];

        for dir in adjacent(c).iter() {
            let next = cur.neighbor(dir);
            if next != prev {
                queue.push_back((next, cur, dist + 1));
            }
        }
    }

    d
}

pub fn adjacent(c: char) -> DirectionSet {
    let mut result = DirectionSet::empty();

    match c {
        '|' => {
            result.insert(Direction::Up);
            result.insert(Direction::Down);
        }

        '-' => {
            result.insert(Direction::Left);
            result.insert(Direction::Right);
        }

        'L' => {
            result.insert(Direction::Up);
            result.insert(Direction::Right);
        }

        'J' => {
            result.insert(Direction::Up);
            result.insert(Direction::Left);
        }

        'F' => {
            result.insert(Direction::Down);
            result.insert(Direction::Right);
        }

        '7' => {
            result.insert(Direction::Down);
            result.insert(Direction::Left);
        }

        '.' | '_' => {}

        'S' => {
            return DirectionSet::all();
        }

        _ => panic!("Unknown tile: {}", c),
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const TEST_INPUT2: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT1);
        assert_eq!(part1(&input), "4");

        let input = crate::parser::parse(TEST_INPUT2);
        assert_eq!(part1(&input), "8");
    }
}
