use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .blueprints
        .iter()
        .map(|bp| bp.number as isize * solve(bp, 24))
        .sum::<isize>()
        .to_string()
}

pub fn solve(blueprint: &Blueprint, time_limit: usize) -> isize {
    let initial_state = State {
        time: 0,
        resources: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        },
        robots: Resources {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        },
    };

    branch_and_bound(
        &initial_state,
        |s: &State| transition(blueprint, s, time_limit),
        |s: &State| {
            if s.time >= time_limit {
                return Some(std::cmp::Reverse(s.resources.geodes));
            }

            None
        },
        |s: &State| {
            let time_left = time_limit - s.time;
            let cur_geodes = s.resources.geodes;
            let cur_production = s.robots.geodes * time_left as isize;
            let potential_production = ((time_left * (time_left.saturating_sub(1))) / 2) as isize;

            std::cmp::Reverse(cur_geodes + cur_production + potential_production)
        },
    )
    .resources
    .geodes
}

pub fn advance(s: &State, new_robots: &Resources, robot_cost: &Resources, result: &mut Vec<State>) {
    result.push(State {
        time: s.time + 1,
        resources: s.resources.clone() + s.robots.clone() - robot_cost.clone(),
        robots: s.robots.clone() + new_robots.clone(),
    })
}

pub fn transition(blueprint: &Blueprint, state: &State, time_limit: usize) -> Vec<State> {
    let mut result = Vec::new();

    let time_remaining = time_limit - state.time;

    if time_remaining == 0 {
        return result;
    }

    if time_remaining > 1 {
        if state.resources.can_afford(&blueprint.geode_robot_cost) {
            advance(
                state,
                &Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geodes: 1,
                },
                &blueprint.geode_robot_cost,
                &mut result,
            );

            return result;
        }

        if state.resources.can_afford(&blueprint.obsidian_robot_cost)
            && state.robots.obsidian < blueprint.geode_robot_cost.obsidian
        {
            advance(
                state,
                &Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 1,
                    geodes: 0,
                },
                &blueprint.obsidian_robot_cost,
                &mut result,
            )
        }

        if state.resources.can_afford(&blueprint.clay_robot_cost)
            && state.robots.clay < blueprint.obsidian_robot_cost.clay
        {
            advance(
                state,
                &Resources {
                    ore: 0,
                    clay: 1,
                    obsidian: 0,
                    geodes: 0,
                },
                &blueprint.clay_robot_cost,
                &mut result,
            )
        }

        if state.resources.can_afford(&blueprint.ore_robot_cost)
            && state.robots.ore
                < blueprint
                    .clay_robot_cost
                    .ore
                    .max(blueprint.obsidian_robot_cost.ore)
                    .max(blueprint.geode_robot_cost.ore)
        {
            advance(
                state,
                &Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
                &blueprint.ore_robot_cost,
                &mut result,
            )
        }
    }

    advance(
        state,
        &Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        },
        &Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        },
        &mut result,
    );

    result
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
