use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    input
        .grid
        .iter()
        .map(|(coord, c)| {
            let h = c;
            let mut score = 1;

            // Not sure if this is sound, but it works.
            if coord.x() == 0 || coord.y() == 0 {
                return 0;
            }

            if coord.x() as usize == input.grid.width() - 1
                || coord.y() as usize == input.grid.height() - 1
            {
                return 0;
            }

            for direction in Direction::all() {
                let mut c = coord;
                let mut d = 0;

                while let Some(tree) = input.grid.get(c + direction.into()) {
                    d += 1;
                    c += direction.into();

                    if tree >= h {
                        break;
                    }
                }

                score *= d.max(1);
            }

            score
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "8");
    }
}
