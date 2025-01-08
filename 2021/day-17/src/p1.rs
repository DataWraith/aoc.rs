use utility_belt::prelude::*;

use crate::{parser::*, quadratic_roots};

pub fn part1(input: &PuzzleInput) -> String {
    let mut ans = 0;

    let (min_vx, max_vx) = x_velocity_bounds(input);
    let (min_vy, max_vy) = y_velocity_bounds(input);

    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            if let Some(max_y) = simulate_trajectory(input, Coordinate::new(vx, vy), ans) {
                ans = ans.max(max_y);
            }
        }
    }

    ans.to_string()
}

pub fn x_velocity_bounds(input: &PuzzleInput) -> (i32, i32) {
    // Because the x-velocity reduces by one at each step, for an initial
    // x-velocity vx, the x-position after n steps is:
    //
    // x(n) = vx + (vx - 1) + (vx - 2) + ... + (vx - (n - 1))
    // x(n) = n * vx - (0 + 1 + 2 + ... + n - 1)
    // x(n) = n * vx - n * (n - 1) / 2
    //
    // for n > vx, we have x(n) = vx * (vx + 1) / 2 because the velocity goes to
    // zero.
    //
    // We want to find the minimum vx such that x(n) >= input.target_area.0.x.
    //
    //    vx * (vx + 1) / 2 >= input.target_area.0.x
    // => vx^2 + vx - 2 * input.target_area.0.x >= 0
    //
    // So we need to find the vx at the point where the parabola intersects
    // the x-axis, which is exactly what the quadratic formula gives us.

    let a = 1.0; // 1 * vx^2
    let b = 1.0; // 1 * vx
    let c = -2.0 * input.target_area.0.x as f64;

    let (x1, x2) = quadratic_roots(a, b, c).unwrap();

    let x = match (x1 > 0.0, x2 > 0.0) {
        (true, true) => x1.min(x2),
        (true, false) => x1,
        (false, true) => x2,
        (false, false) => panic!("No positive roots"),
    };

    // We need to round up the minimum velocity to the nearest integer, because
    // the previous integer is going to undershoot.
    //
    // The maximum x-velocity is the maximum x-value. Any higher than that and
    // we will overshoot in the first step.
    (x.ceil() as i32, input.target_area.1.x)
}

pub fn y_velocity_bounds(input: &PuzzleInput) -> (i32, i32) {
    // The minimum y-velocity is the minimum y-value. Any lower than that and
    // we will undershoot in the first step.
    //
    // For the maximum:
    //
    // When a probe is launched with initial y-velocity vy:
    //
    // - After vy steps, the height is vy * (vy + 1) / 2 (similarly to the x-axis)
    // - Then we spend one step stationary at the peak with velocity 0
    // - After vy more steps, we reach y = 0
    //
    // This means we take a total of 2 * vy + 1 steps to reach the peak and fall
    // back to y=0, so our final velocity at y=0 is vy - (2 * vy + 1) = -vy - 1.
    //
    // We need -vy - 1 to be at least as large as the minimum y-value, otherwise
    // we overshoot the target area when taking a step from y=0.
    //
    //  => -vy - 1 >= input.target_area.0.y
    //  => -vy >= input.target_area.0.y + 1
    //  => vy <= -input.target_area.0.y - 1
    //
    // So the maximum y-velocity is -input.target_area.0.y - 1.

    let y1 = input.target_area.0.y;

    (y1, -y1 - 1)
}

pub fn simulate_trajectory(input: &PuzzleInput, velocity: Coordinate, best_y: i32) -> Option<i32> {
    let mut cur = Coordinate::new(0, 0);
    let mut cur_velocity = velocity;
    let mut max_y = 0;

    loop {
        cur += cur_velocity;
        cur_velocity.x -= cur_velocity.x.signum();
        cur_velocity.y -= 1;

        max_y = cur.y.max(max_y);

        if cur_velocity.y <= 0 && max_y < best_y {
            return None;
        }

        // Did we hit?
        if cur.x >= input.target_area.0.x
            && cur.x <= input.target_area.1.x
            && cur.y >= input.target_area.0.y
            && cur.y <= input.target_area.1.y
        {
            return Some(max_y);
        }

        // Did we undershoot horizontally?
        if cur.x < input.target_area.0.x && cur_velocity.x <= 0 {
            return None;
        }

        // Did we overshoot horizontally?
        if cur.x > input.target_area.1.x && cur_velocity.x >= 0 {
            return None;
        }

        // Did we overshoot vertically?
        if cur.y < input.target_area.0.y && cur_velocity.y <= 0 {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "45");
    }

    #[test]
    fn test_simulate_trajectory() {
        let input = parser::part1(parser::TEST_INPUT);
        assert!(simulate_trajectory(&input, Coordinate::new(7, 2), i32::MIN).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(6, 3), i32::MIN).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(9, 0), i32::MIN).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(17, -4), i32::MIN).is_none());
        assert!(simulate_trajectory(&input, Coordinate::new(6, 9), i32::MIN).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(7, -1), i32::MIN).is_some());
    }
}
