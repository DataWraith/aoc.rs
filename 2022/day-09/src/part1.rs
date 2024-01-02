use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    visited_tiles(input).len().to_string()
}

pub fn step(head: Coordinate, tail: Coordinate, dir: Direction) -> (Coordinate, Coordinate) {
    let new_head = head + dir.into();

    if new_head == tail || new_head.moore_neighbors().contains(&tail) {
        return (new_head, tail);
    }

    if tail.x() == new_head.x() || tail.y() == new_head.y() {
        let new_tail = tail + tail.towards(new_head).into();
        return (new_head, new_tail);
    }

    (new_head, head)
}

pub fn visited_tiles(input: &PuzzleInput) -> HashSet<Coordinate> {
    let mut visited = HashSet::default();

    let mut cur_head = Coordinate::new(0, 0);
    let mut cur_tail = Coordinate::new(0, 0);

    visited.insert(cur_tail);

    for (direction, length) in input.moves.iter() {

        for _ in 0..*length {
            let (new_head, new_tail) = step(cur_head, cur_tail, *direction);

            cur_head = new_head;
            cur_tail = new_tail;

            visited.insert(cur_tail);
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "13");
    }
}
