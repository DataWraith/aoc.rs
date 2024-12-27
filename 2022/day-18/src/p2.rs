use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;

    for (x, y, z) in input.cubes.iter() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
        min_z = min_z.min(*z);
        max_z = max_z.max(*z);
    }

    min_x -= 2;
    max_x += 2;
    min_y -= 2;
    max_y += 2;
    min_z -= 2;
    max_z += 2;

    let mut sets = HashMap::new();
    let mut union_find = UnionFind::default();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if input.cubes.contains(&(x, y, z)) {
                    continue;
                }

                sets.insert((x, y, z), union_find.make_set());
            }
        }
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if input.cubes.contains(&(x, y, z)) {
                    continue;
                }

                for (dx, dy, dz) in [
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 1, 0),
                    (0, -1, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                ] {
                    if x + dx < min_x
                        || x + dx > max_x
                        || y + dy < min_y
                        || y + dy > max_y
                        || z + dz < min_z
                        || z + dz > max_z
                    {
                        continue;
                    }

                    if !input.cubes.contains(&(x + dx, y + dy, z + dz)) {
                        union_find
                            .union(sets[&(x, y, z)], sets[&(x + dx, y + dy, z + dz)])
                            .expect("Union Find failed");
                    }
                }
            }
        }
    }

    let outside = union_find.find(sets[&(min_x, min_y, min_z)]);

    let mut total_sides = 0;

    for (x, y, z) in input.cubes.iter() {
        let mut sides = 6;

        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if x + dx < min_x
                || x + dx > max_x
                || y + dy < min_y
                || y + dy > max_y
                || z + dz < min_z
                || z + dz > max_z
            {
                continue;
            }

            if input.cubes.contains(&(x + dx, y + dy, z + dz)) {
                sides -= 1;
                continue;
            }

            if union_find.find(sets[&(x + dx, y + dy, z + dz)]) != outside {
                sides -= 1;
            }
        }

        total_sides += sides;
    }

    total_sides.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "58");
    }
}
