use crate::{
    bvh::{self, AABB, BVH},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut bvh = BVH::new();

    for brick in input.bricks.iter() {
        bvh.insert(brick.clone());
    }

    let mut bricks = input.bricks.clone();
    bricks.sort_by_key(|b| b.lower_bound.z);

    for brick in bricks.iter() {
        if brick.lower_bound.z == 1 {
            continue;
        }

        let mut z_offset = -1;

        loop {
            let intersect = brick.lower_bound + IVec3::new(0, 0, z_offset);

            let aabb = AABB {
                lower_bound: intersect,
                upper_bound: intersect,
            };

            if intersect.z == 1 || bvh.intersects_any(&aabb) {
                break;
            }

            z_offset -= 1;
        }

        let new_brick = AABB {
            lower_bound: brick.lower_bound + IVec3::new(0, 0, z_offset),
            upper_bound: brick.upper_bound + IVec3::new(0, 0, z_offset),
        };

        let _ = bvh.remove(brick);

        bvh.insert(new_brick);
    }

    dbg!(bvh);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "TODO");
    }
}
