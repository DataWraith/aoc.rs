use std::iter::successors;

use crate::{
    p1::{search, State},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let best = search(input);
    covered(input, &best).to_string()
}

// Counts the number of positions covered by the best path by following the
// path in the best paths.
fn covered(input: &PuzzleInput, best: &[State]) -> usize {
    let mut covered = input.maze.map(|_| false);

    for b in best.iter() {
        for path in b.waypoints.windows(2) {
            let towards = path[0].towards(path[1]);
            covered.set(path[0], true);

            for c in successors(Some(path[0]), |&c| Some(c.neighbor(towards))) {
                covered.set(c, true);
                if c == path[1] {
                    break;
                }
            }
        }
    }

    covered.iter().filter(|(_, &c)| c).count()
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
