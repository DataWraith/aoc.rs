use utility_belt::prelude::*;

use crate::{p1::simulate_trajectory, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut ans = 0;

    for vx in 0..=(input.target_area.1.x * (input.target_area.1.x - 1) / 2) {
        for vy in -1000..=1000 {
            if let Some(_) = simulate_trajectory(input, Coordinate::new(vx, vy)) {
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
    use utility_belt::prelude::*;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "112");
    }
}
