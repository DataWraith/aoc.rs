use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let locks: Vec<_> = input
        .grids
        .iter()
        .filter(|grid| *grid.get((0, 0).into()).unwrap() == '#')
        .map(|grid| count_spaces(grid))
        .map(|counts| invert_spaces(counts))
        .collect();

    let keys: Vec<_> = input
        .grids
        .iter()
        .filter(|grid| *grid.get((0, 0).into()).unwrap() == '.')
        .map(|grid| count_spaces(grid))
        .map(|counts| invert_spaces(counts))
        .collect();

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(l, k)| fits(l, k))
        .count()
        .to_string()
}

fn fits(lock: &[usize], key: &[usize]) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5)
}

fn count_spaces(grid: &Grid2D<char>) -> Vec<usize> {
    grid.col_iter()
        .map(|col| col.iter().filter(|c| **c == '.').count() - 1)
        .collect::<Vec<_>>()
}

fn invert_spaces(counts: Vec<usize>) -> Vec<usize> {
    counts.iter().map(|c| 5 - c).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "3");
    }
}
