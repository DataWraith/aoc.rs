use std::collections::BTreeSet;

use utility_belt::prelude::*;

use crate::{p1::find_regions, parser::*};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for region in find_regions(&input.garden).into_iter() {
        let mut perimeter: BTreeSet<(Coordinate, Direction)> = BTreeSet::new();

        for coord in region.iter() {
            for direction in Direction::cardinal() {
                let neighbor = coord.neighbor(direction);

                if !region.contains(&neighbor) {
                    // Since we started inside the region, we must be just
                    // outside the region now, exactly on one of the sides.
                    perimeter.insert((neighbor, direction));
                }
            }
        }

        let mut sides = 0;

        while let Some((coord, direction)) = perimeter.pop_first() {
            sides += 1;

            // Since coord is on the perimeter and we are pointing outwards, we
            // can turn left/right to walk the side boundary and remove all
            // points on that boundary so that we don't double-count them while
            // iterating over the perimeter.
            let left = direction.turn_left_90();
            let right = direction.turn_right_90();

            let mut cur = coord;

            loop {
                cur += left.into();
                if !perimeter.remove(&(cur, direction)) {
                    break;
                }
            }

            cur = coord;

            loop {
                cur += right.into();
                if !perimeter.remove(&(cur, direction)) {
                    break;
                }
            }
        }

        sum += sides * region.len();
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT4: &str = indoc! {"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"};

    const TEST_INPUT3: &str = indoc! {"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"};

    const TEST_INPUT2: &str = indoc! {"
AAAA
BBCD
BBCC
EEEC
"};

    const TEST_INPUT1: &str = indoc! {"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT1);
        assert_ne!(TEST_INPUT1, "TODO");
        assert_eq!(part2(&input), "1206");
    }

    #[test]
    fn test_part2_example2() {
        let input = crate::parser::part2(TEST_INPUT2);
        assert_ne!(TEST_INPUT2, "TODO");
        assert_eq!(part2(&input), "80");
    }

    #[test]
    fn test_part2_example3() {
        let input = crate::parser::part2(TEST_INPUT3);
        assert_ne!(TEST_INPUT3, "TODO");
        assert_eq!(part2(&input), "436");
    }

    #[test]
    fn test_part2_example4() {
        let input = crate::parser::part2(TEST_INPUT4);
        assert_ne!(TEST_INPUT4, "TODO");
        assert_eq!(part2(&input), "368");
    }
}
