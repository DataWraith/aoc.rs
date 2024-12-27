use utility_belt::prelude::*;

use crate::{p1::NEIGHBOR_OFFSETS, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    // Find the dimensions of the volume containing the lava
    let mut min = IVec3::splat(i32::MAX);
    let mut max = IVec3::splat(i32::MIN);

    for cube in input.cubes.iter() {
        min = min.min(*cube);
        max = max.max(*cube);
    }

    // Add a border of 2 units to make UnionFind easier
    min -= IVec3::new(2, 2, 2);
    max += IVec3::new(2, 2, 2);

    // Prepare UnionFind. We're trying to union all the air spaces to see if
    // they are connected to the outside.
    let mut sets = HashMap::new();
    let mut union_find = UnionFind::default();

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let cur = IVec3::new(x, y, z);

                if input.cubes.contains(&cur) {
                    continue;
                }

                sets.insert(cur, union_find.make_set());
            }
        }
    }

    // Do the actual unioning
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let cur = IVec3::new(x, y, z);

                if input.cubes.contains(&cur) {
                    continue;
                }

                for offset in NEIGHBOR_OFFSETS {
                    let neighbor = cur + offset;

                    if neighbor.x < min.x
                        || neighbor.x > max.x
                        || neighbor.y < min.y
                        || neighbor.y > max.y
                        || neighbor.z < min.z
                        || neighbor.z > max.z
                    {
                        continue;
                    }

                    if !input.cubes.contains(&neighbor) {
                        union_find
                            .union(sets[&cur], sets[&neighbor])
                            .expect("Union Find failed");
                    }
                }
            }
        }
    }

    let outside = union_find.find(sets[&min]);

    let mut total_sides = 0;

    for cube in input.cubes.iter() {
        let mut sides = 6;

        for offset in NEIGHBOR_OFFSETS {
            let neighbor = *cube + offset;

            if input.cubes.contains(&neighbor) {
                sides -= 1;
                continue;
            }

            if union_find.find(sets[&neighbor]) != outside {
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
