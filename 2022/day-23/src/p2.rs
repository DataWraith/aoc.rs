use utility_belt::prelude::*;

use crate::{p1::step, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut elves = input.cells.clone();
    let mut order = VecDeque::from([
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]);

    for i in 1.. {
        let next_elves = step(&elves, &mut order);

        if next_elves == elves {
            return i.to_string();
        }

        elves = next_elves;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "20");
    }
}
