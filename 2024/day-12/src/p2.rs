use utility_belt::prelude::*;

use crate::{
    p1::{find_regions, generate_border},
    parser::*,
};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    let input = input.garden.zoom(3);
    let zoom_divisor = 9;

    for region in find_regions(&input).into_iter() {
        let mut union_find_horizontal = UnionFind::default();
        let mut union_find_vertical = UnionFind::default();

        let mut sets_horizontal = HashMap::new();
        let mut sets_vertical = HashMap::new();

        let border = generate_border(&region);

        for (coord, _count) in border.iter() {
            let set_h = union_find_horizontal.make_set();
            let set_v = union_find_vertical.make_set();

            sets_horizontal.insert(**coord, set_h);
            sets_vertical.insert(**coord, set_v);
        }

        for (coord, _count) in border.iter() {
            let right = coord.neighbor(Direction::Right);

            if border.contains_key(&right) {
                union_find_horizontal
                    .union(sets_horizontal[&coord], sets_horizontal[&right])
                    .expect("Expected union to succeed");
            }

            let down = coord.neighbor(Direction::Down);

            if border.contains_key(&down) {
                union_find_vertical
                    .union(sets_vertical[&coord], sets_vertical[&down])
                    .expect("Expected union to succeed");
            }
        }

        let mut horizontal_count = 0;
        let mut horizontal_counted = HashSet::new();

        for (_coord, set_h) in sets_horizontal.iter() {
            let root = union_find_horizontal.find(*set_h).unwrap();
            let size = union_find_horizontal.size_of_set(root).unwrap_or(0);

            if !horizontal_counted.insert(root) {
                continue;
            }

            if size > 1 {
                horizontal_count += 1;
            }
        }

        let mut vertical_count = 0;
        let mut vertical_counted = HashSet::new();

        for (_coord, set_v) in sets_vertical.iter() {
            let root = union_find_vertical.find(*set_v).unwrap();
            let size = union_find_vertical.size_of_set(root).unwrap_or(0);

            if !vertical_counted.insert(root) {
                continue;
            }

            if size > 1 {
                vertical_count += 1;
            }
        }

        sum += (horizontal_count + vertical_count) * region.len() / zoom_divisor;
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
