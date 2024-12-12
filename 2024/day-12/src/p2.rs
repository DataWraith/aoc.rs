use utility_belt::prelude::*;

use crate::{p1::find_regions3, parser::*};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for region in find_regions3(input).into_iter() {
        let mut border = HashMap::new();

        for coord in region.iter() {
            for neighbor in coord.neighbors() {
                if !region.contains(&neighbor) {
                    border.entry(coord).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }

        let mut sides = 0;

        for row in 0..input.garden.height() {
            for col in 0..input.garden.width() {
                let coord = Coordinate::new(col as i32, row as i32);

                if col + 1 == input.garden.width()
                    || region.contains(&coord)
                        && !region.contains(&coord.neighbor(Direction::Right))
                {
                    sides += 1;
                }
            }
        }

        for row in 0..input.garden.height() {
            for col in 0..input.garden.width() {
                let coord = Coordinate::new(col as i32, row as i32);

                if row + 1 == input.garden.height()
                    || region.contains(&coord) && !region.contains(&coord.neighbor(Direction::Down))
                {
                    sides += 1;
                }
            }
        }

        sum += sides as i32 * region.len() as i32;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
AAAA
BBCD
BBCC
EEEC
"};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part2(&input), "80");
    }
}
