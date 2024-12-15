use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut trajectories = input.robots.clone();

    let mut best_x = (0, u32::MAX);
    let mut best_y = (0, u32::MAX);

    for i in 1..=103usize {
        for robot in trajectories.iter_mut() {
            robot.step();
        }

        // If the robots form the picture, a lot of them are on the same line,
        // horizontally and vertically, so we can check if the sum of distances
        // to the average x or y coordinate is relatively small. In my case, the
        // correct frames have distances of less than 7_000, while all other
        // frames have distances of more than 10_000.
        //
        // We could just loop through all 101 * 103 = 10_403 possible frames to
        // determine the minimum sum, but that takes a few milliseconds.
        //
        // If we find the correct frame for each axis independently, we can use
        // the modular congruences to find the frame where both sequences line
        // up (this only works because 101 and 103 are coprime).
        //
        //   x = a (mod 101)
        //   x = b (mod 103)
        //
        // Where a and b are the frame numbers we determine using the sum of
        // distances to the average, and x is the target frame where both
        // sequences line up. We then solve for x using the Chinese Remainder
        // Theorem.

        let (x_sum, y_sum) = trajectories.iter().fold((0, 0), |(x_sum, y_sum), robot| {
            (x_sum + robot.position.x, y_sum + robot.position.y)
        });

        let x_avg = x_sum / trajectories.len() as i32;
        let y_avg = y_sum / trajectories.len() as i32;

        let (x_diff, y_diff) = trajectories.iter().fold((0, 0), |(x_diff, y_diff), robot| {
            (
                x_diff + robot.position.x.abs_diff(x_avg),
                y_diff + robot.position.y.abs_diff(y_avg),
            )
        });

        if x_diff < best_x.1 {
            best_x = (i, x_diff);
        }

        if y_diff < best_y.1 {
            best_y = (i, y_diff);
        }

        if best_x.1 < 10_000 && best_y.1 < 10_000 {
            break;
        }
    }

    let congruences = [
        Congruence {
            a: best_x.0,
            m: 101,
        },
        Congruence {
            a: best_y.0,
            m: 103,
        },
    ];

    if let Some(solution) = chinese_remainder_theorem(&congruences) {
        return solution.to_string();
    }

    unreachable!("Failed to find solution")
}
