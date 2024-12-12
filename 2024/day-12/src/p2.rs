use utility_belt::prelude::*;

use crate::{p1::find_regions3, parser::*};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for region in find_regions3(&input.garden).into_iter() {
        let mut union_find_horizontal = UnionFind::default();
        let mut union_find_vertical = UnionFind::default();
        let mut sets_horizontal = HashMap::new();
        let mut sets_vertical = HashMap::new();

        let id = input.garden[*region.iter().next().unwrap()];

        let mut border = HashSet::new();

        for coord in region.iter() {
            for neighbor in coord.neighbors() {
                if !region.contains(&neighbor) {
                    border.insert(coord);
                }
            }
        }

        for coord in border.iter() {
            let set_h = union_find_horizontal.make_set();
            let set_v = union_find_vertical.make_set();
            sets_horizontal.insert(**coord, set_h);
            sets_vertical.insert(**coord, set_v);
        }

        for coord in border.iter() {
            let right = coord.neighbor(Direction::Right);

            if border.contains(&right) {
                let _ =
                    union_find_horizontal.union(sets_horizontal[&coord], sets_horizontal[&right]);
            }

            let down = coord.neighbor(Direction::Down);

            if border.contains(&down) {
                let _ = union_find_vertical.union(sets_vertical[&coord], sets_vertical[&down]);
            }
        }

        let horizontal_roots = union_find_horizontal.roots();
        let vertical_roots = union_find_vertical.roots();

        dbg!(&id, horizontal_roots.len(), vertical_roots.len(),);
        sum += (horizontal_roots.len() + vertical_roots.len()) * region.len();
    }

    return sum.to_string();

    let mut sum = 0;
    let input = input.garden.zoom(4);

    dbg!(&input);

    for region in find_regions3(&input).into_iter() {
        let mut border = HashSet::new();

        for coord in region.iter() {
            for neighbor in coord.moore_neighbors() {
                if !region.contains(&neighbor) {
                    border.insert(neighbor);
                }
            }
        }

        let mut coords = border.iter().cloned().collect_vec();
        coords.sort_by_key(|c| (c.y, c.x));

        // RRRRIICCFF
        // RRRRIICCCF
        // VVRRRCCFFF
        // VVRCCCJFFF
        // VVVVCJJCFE
        // VVIVCCJJEE
        // VVIIICJJEE
        // MIIxxxJJEE
        // MIIx_xxEEE
        // MMMx__xEEE
        //    xxxx
        //dbg!(&id);

        if region.len() == 1 {
            sum += 4;
            continue;
        }

        let mut region_sum = 0;
        let mut subtract = 0;

        let mut visited = HashSet::new();

        loop {
            if coords.is_empty() {
                break;
            }

            let mut cur = coords.remove(0);
            let mut cur_dir = Direction::Right;

            if visited.contains(&cur) {
                continue;
            }

            'outer: loop {
                let next = cur.neighbor(cur_dir);

                if !visited.insert(next) {
                    break;
                }

                if border.contains(&next) {
                    visited.insert(cur);
                    cur = next;
                    continue;
                }

                let left = cur_dir.turn_left_90();
                let right = cur_dir.turn_right_90();

                let left_coord = cur.neighbor(left);
                let right_coord = cur.neighbor(right);

                let left_inside = region.contains(&left_coord);
                let left_border = border.contains(&left_coord);
                let right_inside = region.contains(&right_coord);
                let right_border = border.contains(&right_coord);

                if left_border && !right_border {
                    cur_dir = left;
                    region_sum += 1;
                    continue;
                }

                if right_border && !left_border {
                    cur_dir = right;
                    region_sum += 1;
                    continue;
                }

                if left_border && right_border {
                    if visited.contains(&left_coord) && visited.contains(&right_coord) {
                        panic!("both");
                    }

                    if visited.contains(&left_coord) {
                        cur_dir = right;
                    } else {
                        cur_dir = left;
                    }

                    continue;
                }

                panic!("neither");

                /*
                if region.contains(&next) {
                    let left = cur_dir.turn_left_90();
                    cur_dir = left;

                    let left_next = cur.neighbor(left);

                    if !region.contains(&left_next) {
                        region_sum += 1;
                        continue;
                    }
                }

                let right = cur_dir.turn_right_90();
                cur_dir = right;
                region_sum += 1;
                */
            }
        }

        //dbg!(&id, subtract, &region_sum - subtract, region.len() / 16);
        sum += (region_sum - subtract) * region.len() as i32 / 16;
        //dbg!(&sum);
    }

    sum.to_string()
}
// -----1
// EEEEE|
// E----2
// EEEEE
// EXXXX
// EEEEE

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
