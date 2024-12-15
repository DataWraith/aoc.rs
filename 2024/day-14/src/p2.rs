use utility_belt::prelude::*;

use crate::parser::*;

struct Congruence {
    a: u32,
    m: u32,
}

pub fn chinese_remainder_theorem(congruences: &[Congruence]) -> Option<u32> {
    let mut n = 1u32;

    for congruence in congruences {
        assert!(congruence.m != 0);
        debug_assert!(gcd(n, congruence.m) == 1);

        n = n.checked_mul(congruence.m)?;
    }

    let mut solution = 0;

    for congruence in congruences {
        let a_i = congruence.a;
        let m_i = n / congruence.m;
        let n_i = m_i.invm(&congruence.m)?;

        solution = solution.addm(a_i * (m_i % n) * n_i, &n);
    }

    Some(solution)
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut trajectories = input.robots.clone();

    let mut best_x = (0, u32::MAX);
    let mut best_y = (0, u32::MAX);

    for i in 1..=103 {
        for robot in trajectories.iter_mut() {
            robot.step();
        }

        let avg_x =
            trajectories.iter().map(|r| r.position.x).sum::<i32>() / trajectories.len() as i32;
        let avg_y =
            trajectories.iter().map(|r| r.position.y).sum::<i32>() / trajectories.len() as i32;

        let x_diff = trajectories
            .iter()
            .map(|r| r.position.x.abs_diff(avg_x))
            .sum::<u32>();

        let y_diff = trajectories
            .iter()
            .map(|r| r.position.y.abs_diff(avg_y))
            .sum::<u32>();

        if x_diff < best_x.1 {
            best_x = (i, x_diff);
        }

        if y_diff < best_y.1 {
            best_y = (i, y_diff);
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
