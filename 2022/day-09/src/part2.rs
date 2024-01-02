use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    visited_tiles(input, 10).len().to_string()
}

pub fn visited_tiles(input: &PuzzleInput, rope_length: usize) -> HashSet<Coordinate> {
    let mut visited = HashSet::default();

    let mut rope = vec![];
    rope.resize(rope_length, Coordinate::new(0, 0));

    visited.insert(rope[rope_length - 1]);

    for (direction, length) in input.moves.iter() {
        for _ in 0..*length {
            rope[0] += (*direction).into();

            for i in 1..rope_length {
                if rope[i] == rope[i - 1] || rope[i - 1].moore_neighbors().contains(&rope[i]) {
                    break;
                }

                rope[i] = rope[i]
                    .moore_neighbors()
                    .min_by_key(|c| c.manhattan_distance(rope[i - 1]))
                    .unwrap()
            }

            visited.insert(rope[rope_length - 1]);
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
