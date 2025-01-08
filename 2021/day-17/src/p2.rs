use utility_belt::prelude::*;

use crate::{
    p1::{simulate_trajectory, x_velocity_bounds, y_velocity_bounds},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let mut ans = 0;

    let (min_vx, max_vx) = x_velocity_bounds(input);
    let (min_vy, max_vy) = y_velocity_bounds(input);

    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            if simulate_trajectory(input, Coordinate::new(vx, vy), i32::MIN).is_some() {
                ans += 1;
            }
        }
    }

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "112");
    }
}
