
use std::ops::RangeInclusive;

use glam::I64Vec2;
use glam::Vec3Swizzles;
use num::BigRational;
use num::FromPrimitive;
use num::ToPrimitive;
use utility_belt::prelude::Itertools;

use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    solve(
        input,
        200000000000000.0..=400000000000000.0,
        200000000000000.0..=400000000000000.0,
    )
    .to_string()
}

pub fn solve(
    input: &PuzzleInput,
    x_envelope: RangeInclusive<f64>,
    y_envelope: RangeInclusive<f64>,
) -> usize {
    input
        .hailstones
        .iter()
        .combinations(2)
        .filter_map(|c| {
            let a = (c[0].position.xy(), c[0].position.xy() + c[0].velocity.xy());

            let b = (c[1].position.xy(), c[1].position.xy() + c[1].velocity.xy());

            future_line_intersection(a, b)
        })
        .filter(|p| x_envelope.contains(&p.x) && y_envelope.contains(&p.y))
        .count()
}

// This returns the intersection point of two lines, a and b, defined by two points each.
//
// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
pub fn line_intersection_point(
    a: (I64Vec2, I64Vec2),
    b: (I64Vec2, I64Vec2),
) -> Option<glam::DVec2> {
    let x1 = BigRational::from_i64(a.0.x).unwrap();
    let x2 = BigRational::from_i64(a.1.x).unwrap();
    let x3 = BigRational::from_i64(b.0.x).unwrap();
    let x4 = BigRational::from_i64(b.1.x).unwrap();

    let y1 = BigRational::from_i64(a.0.y).unwrap();
    let y2 = BigRational::from_i64(a.1.y).unwrap();
    let y3 = BigRational::from_i64(b.0.y).unwrap();
    let y4 = BigRational::from_i64(b.1.y).unwrap();

    let denominator = (x1.clone() - x2.clone()) * (y3.clone() - y4.clone())
        - (y1.clone() - y2.clone()) * (x3.clone() - x4.clone());

    if denominator == BigRational::from_i64(0).unwrap() {
        return None;
    }

    let numerator_x = (x1.clone() * y2.clone() - y1.clone() * x2.clone())
        * (x3.clone() - x4.clone())
        - (x1.clone() - x2.clone()) * (x3.clone() * y4.clone() - y3.clone() * x4.clone());

    let numerator_y = (x1.clone() * y2.clone() - y1.clone() * x2.clone())
        * (y3.clone() - y4.clone())
        - (y1.clone() - y2.clone()) * (x3.clone() * y4.clone() - y3.clone() * x4.clone());

    Some(glam::DVec2::new(
        (numerator_x / denominator.clone()).to_f64().unwrap(),
        (numerator_y / denominator).to_f64().unwrap(),
    ))
}

pub fn future_line_intersection(
    a: (I64Vec2, I64Vec2),
    b: (I64Vec2, I64Vec2),
) -> Option<glam::DVec2> {
    let intersection_point = line_intersection_point(a, b);

    if let Some(p) = intersection_point {
        let mut in_future = true;

        // The crossing is in the future if both hailstones move towards the intersection point,
        // that is, the distance before taking a step along their trajectory is larger than the
        // distance after taking a step along their trajectory.
        let va = a.1.x - a.0.x;
        let vb = b.1.x - b.0.x;

        in_future =
            in_future && (p.x - a.0.x as f64).abs() > (p.x - (a.0.x as f64 + va as f64)).abs();

        in_future =
            in_future && (p.x - b.0.x as f64).abs() > (p.x - (b.0.x as f64 + vb as f64)).abs();

        if in_future {
            return intersection_point;
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
        assert_eq!(solve(&input, 7f64..=27.0, 7f64..=27.0), 2);
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

        let b = (
            input.hailstones[indices.1].position.xy(),
            input.hailstones[indices.1].position.xy() + input.hailstones[indices.1].velocity.xy(),
        );

        let intersection_point = future_line_intersection(a, b);

        assert_eq!(intersection_point.is_some(), expected.is_some());
        assert_eq!(
            intersection_point.map(|p| (
                (p.x * 1000.0).round() / 1000.0,
                (p.y * 1000.0).round() / 1000.0
            )),
            expected.map(|p| (
                (p.0 * 1000.0).round() / 1000.0,
                (p.1 * 1000.0).round() / 1000.0
            ))
        );
    }
}
