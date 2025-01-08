use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    dbg!(&input);

    let mut ans = 0;

    for vx in 1..=input.target_area.1.x {
        for vy in input.target_area.1.y..=1000 {
            if let Some(max_y) = simulate_trajectory(input, Coordinate::new(vx, vy)) {
                ans = ans.max(max_y);
            }
        }
    }

    ans.to_string()
}

pub fn simulate_trajectory(input: &PuzzleInput, velocity: Coordinate) -> Option<i32> {
    let mut cur = Coordinate::new(0, 0);
    let mut cur_velocity = velocity;
    let mut max_y = 0;

    loop {
        cur += cur_velocity;
        cur_velocity.x -= cur_velocity.x.signum();
        cur_velocity.y -= 1;

        max_y = cur.y.max(max_y);

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
        if cur.y < input.target_area.1.y && cur_velocity.y <= 0 {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "45");
    }

    #[test]
    fn test_simulate_trajectory() {
        let input = parser::part1(parser::TEST_INPUT);
        assert!(simulate_trajectory(&input, Coordinate::new(7, 2)).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(6, 3)).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(9, 0)).is_some());
        assert!(simulate_trajectory(&input, Coordinate::new(17, -4)).is_none());
        assert!(simulate_trajectory(&input, Coordinate::new(6, 9)).is_some());
    }
}
