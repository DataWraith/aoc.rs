use crate::{
    bvh::{AABB, BVH},
    part1::{apply_gravity, find_bricks_supported_by},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut bvh = BVH::new();

    for brick in input.bricks.iter() {
        bvh.insert(brick.clone());
    }

    let mut count = 0;

    let bricks = apply_gravity(
        &input.bricks.iter().cloned().enumerate().collect::<Vec<_>>(),
        &mut bvh,
    );

    let mut prev = HashSet::default();

    for b in bricks.iter() {
        prev.insert(b);
    }

    for (i, brick) in bricks.iter() {
        let mut b = bvh.clone();

        b.remove(brick);

        let afterwards = apply_gravity(&bricks, &mut b);

        for a_brick in afterwards.into_iter() {
            if prev.contains(&a_brick) {
                continue;
            }

            count += 1;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "7");
    }
}
