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

pub fn advance(
    s: &State,
    time_passed: usize,
    new_robots: &Resources,
    robot_cost: &Resources,
    result: &mut Vec<State>,
) {
    result.push(State {
        time: s.time + time_passed,
        resources: s.resources.clone() + s.robots.clone() * time_passed - robot_cost.clone(),
        robots: s.robots.clone() + new_robots.clone(),
    })
}

pub fn prune_resources(blueprint: &Blueprint, state: &State, time_limit: usize) -> State {
    let time_remaining = (time_limit - state.time) as isize;

    let max_spendable_obsidian = time_remaining * blueprint.max_resources.obsidian;
    let max_spendable_clay = time_remaining * blueprint.max_resources.clay;
    let max_spendable_ore = time_remaining * blueprint.max_resources.ore;

    State {
        resources: Resources {
            ore: state.resources.ore.min(max_spendable_ore),
            clay: state.resources.clay.min(max_spendable_clay),
            obsidian: state.resources.obsidian.min(max_spendable_obsidian),
            geodes: state.resources.geodes,
        },
        ..state.clone()
    }
}

pub fn wait_time(cost: &Resources, state: &State) -> usize {
    fn ceil_divide(a: isize, b: isize) -> isize {
        if a == 0 {
            return 0;
        }

        if b == 0 {
            return isize::MAX;
        }

        (a + b - 1) / b
    }

    let obsidian_wait = ceil_divide(
        cost.obsidian.saturating_sub(state.resources.obsidian),
        state.robots.obsidian,
    );

    let clay_wait = ceil_divide(
        cost.clay.saturating_sub(state.resources.clay),
        state.robots.clay,
    );

    let ore_wait = ceil_divide(
        cost.ore.saturating_sub(state.resources.ore),
        state.robots.ore,
    );

    obsidian_wait.max(clay_wait).max(ore_wait) as usize
}

pub fn transition(blueprint: &Blueprint, state: &State, time_limit: usize) -> Vec<State> {
    let mut result = Vec::new();

    let time_remaining = time_limit - state.time;

    if time_remaining == 0 {
        return result;
    }

    if time_remaining > 1 {
        for r in 0..4 {
            let wait = wait_time(&blueprint.robot_costs[r], state);

            let robot_mines = Resources {
                ore: (r == 0) as isize,
                clay: (r == 1) as isize,
                obsidian: (r == 2) as isize,
                geodes: (r == 3) as isize,
            };

            if wait < time_remaining
                && (state.robots.clone() + robot_mines.clone())[r] <= blueprint.max_resources[r]
            {
                advance(
                    state,
                    1 + wait,
                    &robot_mines,
                    &blueprint.robot_costs[r],
                    &mut result,
                );
            }

            if wait == 0 && r == 3 {
                break;
            }
        }
    }

    if result.is_empty() {
        advance(
            state,
            time_remaining,
            &Resources::default(),
            &Resources::default(),
            &mut result,
        );
    }

    result
        .iter()
        .map(|s| prune_resources(blueprint, s, time_limit))
        .collect_vec()
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
