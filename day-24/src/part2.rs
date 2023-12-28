use glam::I64Vec2;
use glam::Vec3Swizzles;

use num::traits::FromPrimitive;
use num::BigRational;
use utility_belt::prelude::*;

use crate::{part1::future_line_intersection, structs::*};

pub fn part2(input: &PuzzleInput) -> String {
    // See https://www.youtube.com/watch?v=guOyA7Ijqgk for a really elegant solution to this problem.
    // Unfortunately, it appears that there is no equivalent symbolic Algebra library for Rust, so we're stuck
    // with a slow grid search.

    let (final_x, final_y) = solve_2d(input).unwrap();

    let input2 = PuzzleInput {
        hailstones: input
            .hailstones
            .iter()
            .map(|h| Hailstone {
                position: h.position.xzy(),
                velocity: h.velocity.xzy(),
            })
            .collect_vec(),
    };

    let (x2, final_z) = solve_2d(&input2).unwrap();

    assert_eq!(final_x, x2);

    (final_x + final_y + final_z).to_string()
}

pub fn solve_2d(input: &PuzzleInput) -> Option<(BigRational, BigRational)> {
    // The idea here is to do a grid search for the correct velocity in the x/y
    // or x/z plane. We subtract the chosen velocity from the hailstones, which
    // basically shifts our frame of reference.
    //
    // Since we know that all hailstones _must_ line up at some point, we can
    // check for that and then solve for the origin, which is simply the
    // (future) collision point for all hailstones. I'm still having trouble
    // visualizing this...

    for x in -300..=300 {
        dbg!(x);
        for y in -300..=300 {
            let hailstones = input
                .hailstones
                .iter()
                .map(|h| {
                    (
                        h.position.xy(),
                        // NOTE: We add the position here, because the intersection functions don't take a velocity,
                        // but two points on the line.
                        h.position.xy() + h.velocity.xy() - I64Vec2::new(x, y),
                    )
                })
                .map(|(p, v)| {
                    (
                        (
                            BigRational::from_i64(p.x).unwrap(),
                            BigRational::from_i64(p.y).unwrap(),
                        ),
                        (
                            BigRational::from_i64(v.x).unwrap(),
                            BigRational::from_i64(v.y).unwrap(),
                        ),
                    )
                })
                .collect_vec();

            if let Some(initial_collision) =
                future_line_intersection(hailstones[0].clone(), hailstones[1].clone())
            {
                if hailstones.iter().skip(2).all(|h| {
                    if let Some(collision) =
                        future_line_intersection(hailstones[0].clone(), h.clone())
                    {
                        collision == initial_collision
                    } else {
                        false
                    }
                }) {
                    return Some(initial_collision);
                }
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "47");
    }
}
