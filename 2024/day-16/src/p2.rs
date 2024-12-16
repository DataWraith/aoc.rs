use std::iter::successors;
use utility_belt::prelude::*;

use crate::{
    p1::{follow_path, junctions, race, State},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let mut tabu = HashSet::new();

    let mut best = race(input, &tabu);
    todo!();
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "45");
    }
}
