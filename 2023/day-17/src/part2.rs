use crate::{part1::solve, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let start = Crucible {
        direction: None,
        position: Coordinate::new(0, 0),
        cur_straight: 0,
        min_straight: 4,
        max_straight: 10,
    };

    solve(input, start).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    const TEST_INPUT2: &str = indoc! {"
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT2);
        assert_eq!(part2(&input), "71");

        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "94");
    }
}
