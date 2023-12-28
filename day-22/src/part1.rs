use crate::{
    bvh::{AABB, BVH},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut bvh = BVH::new();

    for brick in input.bricks.iter() {
        bvh.insert(brick.clone());
    }

    let bricks = apply_gravity(
        &input.bricks.iter().cloned().enumerate().collect::<Vec<_>>(),
        &mut bvh,
    );

    let mut supporting_bricks = Vec::new();

    for brick in bricks.iter() {
        let resting = find_bricks_supported_by(&bvh, &brick.1);

        if resting.is_empty() {
            continue;
        }

        supporting_bricks.push(brick);
    }

    (bricks.len() - supporting_bricks.len()).to_string()
}

// This checks for intersection with the given brick translated one unit higher in the z direction.
pub fn find_bricks_supported_by(bvh: &BVH, brick: &AABB) -> Vec<AABB> {
    let mut bricks = Vec::new();

    let intersector = AABB {
        lower_bound: brick.lower_bound + IVec3::new(0, 0, 1),
        upper_bound: brick.upper_bound + IVec3::new(0, 0, 1),
    };

    let resting = bvh.all_intersecting_leaves(&intersector);

    for r in resting.into_iter() {
        if r == *brick {
            continue;
        }

        let mut r2 = r.clone();

        r2.lower_bound.z -= 1;
        r2.upper_bound.z -= 1;

        if bvh
            .all_intersecting_leaves(&r2)
            .into_iter()
            .filter(|resting| *resting != r && *resting != *brick)
            .count()
            == 0
        {
            bricks.push(r);
        }
    }

    bricks
}

pub fn apply_gravity(bricks: &[(usize, AABB)], bvh: &mut BVH) -> Vec<(usize, AABB)> {
    let mut result = Vec::new();

    let mut bricks = bricks.to_vec();
    bricks.sort_by_key(|b| b.1.lower_bound.z);

    for brick in bricks.iter() {
        if brick.1.lower_bound.z == 1 {
            result.push(brick.clone());
            continue;
        }

        let mut z_offset = -1;

        loop {
            let intersect_lower = brick.1.lower_bound + IVec3::new(0, 0, z_offset);
            let mut intersect_upper = brick.1.upper_bound;
            intersect_upper.z = intersect_lower.z;

            let aabb = AABB {
                lower_bound: intersect_lower,
                upper_bound: intersect_upper,
            };

            if bvh.intersects_any(&aabb) {
                break;
            }

            if intersect_lower.z == 0 {
                break;
            }

            z_offset -= 1;
        }

        z_offset += 1;

        let new_brick = AABB {
            lower_bound: brick.1.lower_bound + IVec3::new(0, 0, z_offset),
            upper_bound: brick.1.upper_bound + IVec3::new(0, 0, z_offset),
        };

        if brick.1 != new_brick {
            result.push((brick.0, new_brick.clone()));
            let _ = bvh.remove(&brick.1);
            bvh.insert(new_brick);
        } else {
            result.push(brick.clone());
        }
    }

    result
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "5");
    }
}
