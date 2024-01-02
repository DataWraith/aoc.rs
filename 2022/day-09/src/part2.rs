use crate::{part1::step, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    visited_tiles(input).len().to_string()
}

pub fn visited_tiles(input: &PuzzleInput) -> HashSet<Coordinate> {
    let mut visited = HashSet::default();

    let mut rope = [Coordinate::new(0, 0); 10];

    visited.insert(rope[9]);

    for (direction, length) in input.moves.iter() {
        for _ in 0..*length {
            let mut prev_rope = rope[0];

            rope[0] += (*direction).into();

            for i in 1..10 {
                if rope[i] == rope[i - 1] || rope[i - 1].moore_neighbors().contains(&rope[i]) {
                    break;
                }

                if rope[i].x() == rope[i - 1].x() || rope[i].y() == rope[i - 1].y() {
                    rope[i] += rope[i].towards(rope[i - 1]).into();
                } else {
                    rope[i] = rope[i]
                        .moore_neighbors()
                        .min_by_key(|c| c.manhattan_distance(rope[i - 1]))
                        .unwrap()
                        .clone();
                }
            }

            visited.insert(rope[9]);
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test2.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "36");
    }
}
