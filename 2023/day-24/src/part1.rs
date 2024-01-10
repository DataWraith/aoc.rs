use std::ops::RangeInclusive;

use glam::Vec3Swizzles;
use num::BigRational;
use num::Signed;
use num::Zero;
use num::{FromPrimitive, ToPrimitive};

use utility_belt::math::line_intersection_point;
use utility_belt::prelude::Itertools;

use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    solve(
        input,
        (BigRational::from_u64(200000000000000).unwrap())
            ..=(BigRational::from_u64(400000000000000).unwrap()),
        (BigRational::from_u64(200000000000000).unwrap())
            ..=(BigRational::from_u64(400000000000000).unwrap()),
    )
    .to_string()
}

pub fn solve(
    input: &PuzzleInput,
    x_envelope: RangeInclusive<BigRational>,
    y_envelope: RangeInclusive<BigRational>,
) -> usize {
    input
        .hailstones
        .iter()
        .combinations(2)
        .filter_map(|c| {
            let a = (c[0].position.xy(), c[0].position.xy() + c[0].velocity.xy());
            let a_x = (
                BigRational::from_i64(a.0.x).unwrap(),
                BigRational::from_i64(a.0.y).unwrap(),
            );
            let a_y = (
                BigRational::from_i64(a.1.x).unwrap(),
                BigRational::from_i64(a.1.y).unwrap(),
            );

            let b = (c[1].position.xy(), c[1].position.xy() + c[1].velocity.xy());
            let b_x = (
                BigRational::from_i64(b.0.x).unwrap(),
                BigRational::from_i64(b.0.y).unwrap(),
            );
            let b_y = (
                BigRational::from_i64(b.1.x).unwrap(),
                BigRational::from_i64(b.1.y).unwrap(),
            );

            future_line_intersection((a_x, a_y), (b_x, b_y))
        })
        .filter(|p| x_envelope.contains(&p.0) && y_envelope.contains(&p.1))
        .count()
}

pub fn future_line_intersection(
    a: ((BigRational, BigRational), (BigRational, BigRational)),
    b: ((BigRational, BigRational), (BigRational, BigRational)),
) -> Option<(BigRational, BigRational)> {
    let intersection_point = line_intersection_point(a.clone(), b.clone(), BigRational::zero());

    if let Some(p) = intersection_point {
        let mut in_future = true;

        // The crossing is in the future if both hailstones move towards the intersection point,
        // that is, the distance before taking a step along their trajectory is larger than the
        // distance after taking a step along their trajectory.
        let va = (a.1).0.clone() - (a.0).0.clone();
        let vb = (b.1).0.clone() - (b.0).0.clone();

        in_future = in_future
            && (p.0.clone() - a.0.clone().0).abs() > (p.0.clone() - (a.0.clone().0 + va)).abs();
        in_future = in_future
            && (p.0.clone() - b.0.clone().0).abs() > (p.0.clone() - (b.0.clone().0 + vb)).abs();

        if in_future {
            return Some(p);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;
    use utility_belt::prelude::rstest::*;

    const TEST_INPUT: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        let range = BigRational::from_u64(7).unwrap()..=BigRational::from_u64(27).unwrap();
        assert_eq!(solve(&input, range.clone(), range.clone()), 2);
    }

    #[rstest]
    #[case((0, 1), Some((14.3333, 15.3333)))]
    #[case((0, 2), Some((11.6667, 16.6667)))]
    #[case((0, 3), Some((6.2, 19.4)))]
    #[case((0, 4), None)]
    #[case((1, 2), None)]
    #[case((1, 3), Some((-6., -5.)))]
    #[case((1, 4), None)]
    #[case((2, 3), Some((-2.0, 3.0)))]
    #[case((2, 4), None)]
    #[case((3, 4), None)]
    fn test_intersection(#[case] indices: (usize, usize), #[case] expected: Option<(f64, f64)>) {
        let input = crate::parser::parse(TEST_INPUT);

        let a = (
            input.hailstones[indices.0].position.xy(),
            input.hailstones[indices.0].position.xy() + input.hailstones[indices.0].velocity.xy(),
        );

        let a = (
            (
                BigRational::from_i64(a.0.x).unwrap(),
                BigRational::from_i64(a.0.y).unwrap(),
            ),
            (
                BigRational::from_i64(a.1.x).unwrap(),
                BigRational::from_i64(a.1.y).unwrap(),
            ),
        );

        let b = (
            input.hailstones[indices.1].position.xy(),
            input.hailstones[indices.1].position.xy() + input.hailstones[indices.1].velocity.xy(),
        );

        let b = (
            (
                BigRational::from_i64(b.0.x).unwrap(),
                BigRational::from_i64(b.0.y).unwrap(),
            ),
            (
                BigRational::from_i64(b.1.x).unwrap(),
                BigRational::from_i64(b.1.y).unwrap(),
            ),
        );

        let intersection_point = future_line_intersection(a, b);

        assert_eq!(intersection_point.is_some(), expected.is_some());
        assert_eq!(
            intersection_point.map(|p| (
                (p.0.to_f64().unwrap() * 1000.0).round() / 1000.0,
                (p.1.to_f64().unwrap() * 1000.0).round() / 1000.0
            )),
            expected.map(|p| (
                (p.0 * 1000.0).round() / 1000.0,
                (p.1 * 1000.0).round() / 1000.0
            ))
        );
    }
}
