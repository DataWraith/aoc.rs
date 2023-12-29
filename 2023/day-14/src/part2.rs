use crate::{
    part1::{load, tilt},
    structs::*,
};

use utility_belt::{misc::path_contraction, prelude::*};

pub fn part2(input: &PuzzleInput, cycles: usize) -> String {
    load(&path_contraction(input, cycle, cycles)).to_string()
}

pub fn cycle(input: &PuzzleInput) -> PuzzleInput {
    let north = tilt(input, Direction::Up);
    let west = tilt(&north, Direction::Left);
    let south = tilt(&west, Direction::Down);

    tilt(&south, Direction::Right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    const ONE_CYCLE: &str = indoc! {"
        .....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#....
    "};

    const TWO_CYCLE: &str = indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O
    "};

    const THREE_CYCLE: &str = indoc! {"
        .....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #...O###.O
        #.OOO#...O
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input, 1000000000), "64");
    }

    #[test]
    fn cycle_test() {
        let input = crate::parser::parse(TEST_INPUT);

        let expected = crate::parser::parse(ONE_CYCLE);
        let expected2 = crate::parser::parse(TWO_CYCLE);
        let expected3 = crate::parser::parse(THREE_CYCLE);

        let processed = cycle(&input.clone());
        let processed2 = cycle(&processed.clone());
        let processed3 = cycle(&processed2.clone());

        assert_eq!(processed, expected);
        assert_eq!(processed2, expected2);
        assert_eq!(processed3, expected3);
    }
}
