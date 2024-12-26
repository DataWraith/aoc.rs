use crate::{part1::register_sequence, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let xs = register_sequence(input);

    let mut cur = Coordinate::new(0, 0);
    let mut screen = Grid2D::new(40, 6, '.');

    for (i, register) in xs.iter().enumerate().take(40 * 6) {
        let x = i % 40;

        if (x as isize).abs_diff(*register) < 2 {
            screen[cur] = '#';
        }

        cur += Direction::Right;

        if cur.x >= screen.width() as i32 {
            cur = Coordinate::new(0, cur.y + 1);
        }
    }

    format!("{}", screen)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const EXPECTED: &str = include_str!("../expected.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "\n".to_owned() + EXPECTED + "\n");
    }
}
