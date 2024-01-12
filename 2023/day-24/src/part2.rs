use glam::Vec3Swizzles;

use ndarray::Array1;
use num::traits::ToPrimitive;
use num::BigRational;
use utility_belt::prelude::*;
use z3::{ast::Ast, ast::Int, SatResult, Solver};

use crate::structs::*;

pub fn part2_z3(input: &PuzzleInput) -> String {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    let sx = Int::new_const(&ctx, "sx");
    let sy = Int::new_const(&ctx, "sy");
    let sz = Int::new_const(&ctx, "sz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    let solver = Solver::new(&ctx);

    for h in input.hailstones.iter().take(3) {
        let hx = Int::from_i64(&ctx, h.position.x);
        let hy = Int::from_i64(&ctx, h.position.y);
        let hz = Int::from_i64(&ctx, h.position.z);
        let hvx = Int::from_i64(&ctx, h.velocity.x);
        let hvy = Int::from_i64(&ctx, h.velocity.y);
        let hvz = Int::from_i64(&ctx, h.velocity.z);
        let ht = Int::fresh_const(&ctx, "ht");

        solver.assert(&(&sx + &vx * &ht - &hx - &hvx * &ht)._eq(&Int::from_i64(&ctx, 0)));
        solver.assert(&(&sy + &vy * &ht - &hy - &hvy * &ht)._eq(&Int::from_i64(&ctx, 0)));
        solver.assert(&(&sz + &vz * &ht - &hz - &hvz * &ht)._eq(&Int::from_i64(&ctx, 0)));
    }

    assert_eq!(solver.check(), SatResult::Sat);

    let sx = solver
        .get_model()
        .unwrap()
        .eval(&sx, true)
        .unwrap()
        .as_i64()
        .unwrap();

    let sy = solver
        .get_model()
        .unwrap()
        .eval(&sy, true)
        .unwrap()
        .as_i64()
        .unwrap();

    let sz = solver
        .get_model()
        .unwrap()
        .eval(&sz, true)
        .unwrap()
        .as_i64()
        .unwrap();

    (sx + sy + sz).to_string()
}

#[allow(dead_code)]
pub fn part2(input: &PuzzleInput) -> String {
    // This can be solved by solving a system of linear equations:
    //
    // We have six unknowns:
    //
    // - sx, sy, sz: the position of the starting point
    // - vx, vy, vz: the velocity of the rock
    //
    // We want to devise a system of linear equations that can be solved for
    // these unknowns. We have the following equations at our disposal:
    //
    // sx + (t * vx) = hx + (t * hvx)
    // sy + (t * vy) = hy + (t * hvy)
    // sz + (t * vz) = hz + (t * hvz)
    //
    // We can rearrange these equations to the following form:
    //
    //    sx + t * vx = hx + t * hvx
    // => sx - hx + t * vx = t * hvx
    // => sx - hx = t * hvx - t * vx
    // => sx - hx = t * (hvx - vx)
    // => (sx - hx) / (hvx - vx) = t
    //
    // Same with the other coordinates, so:
    //
    // t = (sx - hx) / (hvx - vx)
    // t = (sy - hy) / (hvy - vy)
    // t = (sz - hz) / (hvz - vz)
    //
    // Let's try equating the first two.
    //
    //    (sx - hx) / (hvx - vx) = (sy - hy) / (hvy - vy)
    // => (sx - hx) * (hvy - vy) = (sy - hy) * (hvx - vx)
    // => sx * hvy - sx * vy - hx * hvy + hx * vy = sy * hvx - sy * vx - hy * hvx + hy * vx
    // => sx * hvy - sx * vy = sy * hvx - sy * vx - hy * hvx + hy * vx + hx * hvy - hx * vy
    // => sx * hvy = sy * hvx - sy * vx - hy * hvx + hy * vx + hx * hvy - hx * vy + sx * vy
    // => sy * hvx - sy * vx - hy * hvx + hy * vx + hx * hvy - hx * vy + sx * vy - sx * hvy = 0
    //
    // Now we want to isolate the rock variables, because those are the ones we
    // want to solve for.
    //
    //    sy * hvx - sy * vx - hy * hvx + hy * vx + hx * hvy - hx * vy + sx * vy - sx * hvy = 0
    // => sy * hvx - sy * vx - hy * hvx + hy * vx + hx * hvy - hx * vy - sx * hvy = -sx * vy
    // => sy * hvx - hy * hvx + hy * vx + hx * hvy - hx * vy - sx * hvy = sy * vx - sx * vy
    //
    // Note that the right hand side only depends on the rock's start position
    // and velocity; there is no dependency on the hailstone or the time. That
    // means we can substitute any other hailstone's values into the equation
    // and it will still be true -- the rock will hit that hailstone too if
    // we find the right starting point and velocity. That in turn means that
    // we can replace the right hand side with another hailstone's equation,
    // because they must also equal the right hand side.
    //
    // =>   sy * h1vx - h1y * h1vx + h1y * vx + h1x * h1vy - h1x * vy - sx * h1vy
    //    = sy * h2vx - h2y * h2vx + h2y * vx + h2x * h2vy - h2x * vy - sx * h2vy
    //
    // =>  sy * h1vx - sy * h2vx - h1y * h1vx + h2y + h2vx + h1y * vx - h2y * vx + h1x * h1vy - h2x * h2vy - h1x * vy + h2x * vy - sx * h1vy + sx * h2vy = 0
    // =>  sy * (h1vx - h2vx) - h1y * h1vx + h2y * h2vx + vx * (h1y - h2y) + h1x * h1vy - h2x * h2vy - vy * (h1x - h2x) - sx * (h1vy - h2vy) = 0
    // =>  sx * (h2vy - h1vy) + sy * (h1vx - h2vx) + vx * (h1y - h2y) + vy * (h2x - h1x) = h1y * h1vx - h1x * h1vy - h2y * h2vx + h2x * h2vy
    //
    // And that can be solved using Gauss-Jordan elimination, because each
    // unknown variable is only defined in terms of values we know.
    let (sx, sy, vx, _vy) = solve2d(input);

    // We could do more algebra to deduce the z coordinate (that would be more
    // elegant), but it is easier to just switch the y and z coordinates and
    // solve the same system of equations again.
    let input2 = PuzzleInput {
        hailstones: input
            .hailstones
            .iter()
            .map(|h| Hailstone {
                position: h.position.xzy(),
                velocity: h.velocity.xzy(),
            })
            .collect::<Vec<_>>(),
    };

    let (sx2, sz, vx2, _vz) = solve2d(&input2);

    assert_eq!(sx, sx2);
    assert_eq!(vx, vx2);

    (sx + sy + sz).to_string()
}

#[allow(dead_code)]
fn solve2d(input: &PuzzleInput) -> (i64, i64, i64, i64) {
    let mut matrix = Array2::zeros((10, 5));
    let mut idx = 0;

    for i in 0..5 {
        for j in (i + 1)..5 {
            let h1 = &input.hailstones[i];
            let h2 = &input.hailstones[j];

            matrix[[idx, 0]] = BigRational::new((h2.velocity.y - h1.velocity.y).into(), 1.into());
            matrix[[idx, 1]] = BigRational::new((h1.velocity.x - h2.velocity.x).into(), 1.into());
            matrix[[idx, 2]] = BigRational::new((h1.position.y - h2.position.y).into(), 1.into());
            matrix[[idx, 3]] = BigRational::new((h2.position.x - h1.position.x).into(), 1.into());

            matrix[[idx, 4]] = BigRational::new(
                ((h1.position.y * h1.velocity.x)
                    + (-h1.position.x * h1.velocity.y)
                    + (-h2.position.y * h2.velocity.x)
                    + (h2.position.x * h2.velocity.y))
                    .into(),
                1.into(),
            );

            idx += 1;
        }
    }

    let mut ans = Array1::zeros(4);
    let zero = BigRational::new(0.into(), 1.into());
    let soln = gauss_jordan(matrix, &mut ans, zero);

    assert_eq!(soln, Solution::Unique);
    ans = ans.map(|x| x.round());

    let sx = ans[0].to_i64().unwrap();
    let sy = ans[1].to_i64().unwrap();
    let vx = ans[2].to_i64().unwrap();
    let vy = ans[3].to_i64().unwrap();

    (sx, sy, vx, vy)
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
        assert_eq!(part2_z3(&input), "47");
    }
}
