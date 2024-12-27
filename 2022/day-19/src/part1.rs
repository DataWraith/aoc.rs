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

        // Prune by comparing against the best result so far (branch and bound).
        // Our upper bound starts as the sum of the geodes we have and the
        // geodes we can produce with the robots we have in the remaining time.
        //
        // To get a true upper bound, we also assume that we can produce one
        // more geode robot per turn, which is the maximum possible.
        //
        // A robot produced when `n` turns are remaining will produce `n - 1`
        // geodes, which leads to the series sum of `n - 1 + n - 2 + ... + 1`,
        // which is `n * (n - 1) / 2`.
        let n = state.time_remaining as u32;
        let upper_bound = state.pack.resources.geodes as u32 + state.pack.robots.geodes as u32 * n;
        let upper_bound = upper_bound + n * (n - 1) / 2;

        if upper_bound <= *max_result as u32 {
            return 0;
        }

        // We might be able to beat our current best result, so we try to build
        // each robot in turn and see if we can do better, most advanced to
        // least advanced.
        let mut result = 0;
        let mut built_geode_robot = false;

        for (robot, cost) in blueprint.robot_costs.iter().enumerate().rev() {
            // Check if we can afford the robot.
            //
            // Also, we don't want to build more robots than we need to produce
            // the most expensive robot, because that would just be wasteful.
            if state.pack.can_afford(cost)
                && state.pack.robots[robot] < blueprint.max_resources[robot]
            {
                // If we can build a geode robot, we always try to build it
                // first. Otherwise, we try to build each robot we are allowed
                // to build in turn. Once we build a robot, we also reset the
                // blacklist for the next turn.
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

                // This blacklists the robot for the next turn(s), because it
                // doesn't make sense to build it then, when we could have built
                // it now (except for geode robots -- obviously, we always want
                // to build those, which is why we exit the loop early above
                // in that case).
                new_can_build[robot] = false;
            }
        }

        // If we built a geode robot, we can stop searching, because that is the
        // optimal choice.
        if built_geode_robot {
            return result;
        }

        // Otherwise, we try to build nothing this turn.
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
