use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .blueprints
        .iter()
        .map(|bp| bp.number as isize * max_geodes(bp, 24) as isize)
        .sum::<isize>()
        .to_string()
}

pub fn max_geodes(blueprint: &Blueprint, time_limit: u8) -> u8 {
    fn solution(
        blueprint: &Blueprint,
        state: State,
        max_result: &mut u8,
        can_build: [bool; 3],
    ) -> u8 {
        let mut new_can_build = [true; 3];

        // Done!
        if state.time_remaining == 0 {
            let result = state.pack.resources.geodes;
            *max_result = (*max_result).max(result);
            return result;
        }

        // Prune by comparing against the best result so far.
        let n = state.time_remaining as u32;
        let upper_bound = state.pack.resources.geodes as u32
            + state.pack.robots.geodes as u32 * state.time_remaining as u32;
        let upper_bound = upper_bound + n * (n - 1) / 2;

        if upper_bound <= *max_result as u32 {
            return 0;
        }

        let mut result = 0;
        let mut built_geode_robot = false;

        for (robot, cost) in blueprint.robot_costs.iter().enumerate().rev() {
            if state.pack.can_afford(cost)
                && state.pack.robots[robot] < blueprint.max_resources[robot]
            {
                if robot == 3 || can_build[robot] {
                    result = result.max(solution(
                        blueprint,
                        state.tick().buy_robot(robot, *cost),
                        max_result,
                        [true; 3],
                    ));

                    if robot == 3 {
                        built_geode_robot = true;
                        break;
                    }
                }

                new_can_build[robot] = false;
            }
        }

        if built_geode_robot {
            return result;
        }

        result.max(solution(blueprint, state.tick(), max_result, new_can_build))
    }

    solution(blueprint, State::new(time_limit), &mut 0, [true; 3])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "33");
    }
}
