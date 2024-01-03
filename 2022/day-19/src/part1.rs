use std::{cmp::Reverse, collections::BinaryHeap};

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .blueprints
        .iter()
        .map(|bp| bp.number as isize * simulate(bp, 24))
        .sum::<isize>()
        .to_string()
}

pub fn simulate(blueprint: &Blueprint, time_limit: usize) -> isize {
    let initial_state = State {
        time_remaining: time_limit,
        resources: Resources::default(),
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
            if s.time_remaining == 0 {
                return Some(std::cmp::Reverse(s.resources.geodes));
            }

            None
        },
        |s: &State| {
            let cur_geodes = s.resources.geodes;
            let cur_production = s.robots.geodes * s.time_remaining as isize;
            let potential_production =
                ((s.time_remaining * (s.time_remaining.saturating_sub(1))) / 2) as isize;

            std::cmp::Reverse(cur_geodes + cur_production + potential_production)
        },
    )
    .resources
    .geodes
}

// Generates a successor state with the given parameters and adds it to the result Vec.
pub fn tick(
    s: &State,
    time_passed: usize,
    new_robots: &Resources,
    robot_cost: &Resources,
    result: &mut Vec<State>,
) {
    result.push(State {
        time_remaining: s.time_remaining - time_passed,
        resources: s.resources.clone() + s.robots.clone() * time_passed - robot_cost.clone(),
        robots: s.robots.clone() + new_robots.clone(),
    })
}

// This discards all resources that we can't possibly spend in the remaining
// time, reducing the number of duplicate states and approximately halving the
// time needed for the search.
pub fn prune_resources(blueprint: &Blueprint, state: &State) -> State {
    let time_remaining = state.time_remaining as isize;

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

// Calculate how long we have to wait for a robot to become available at the current rate
// of resource production. Returns usize::MAX if the robot will never become available.
pub fn wait_time(cost: &Resources, state: &State) -> usize {
    // Divide and then round up
    fn ceil_divide(a: usize, b: usize) -> usize {
        if a == 0 {
            return 0;
        }

        if b == 0 {
            return usize::MAX;
        }

        (a + b - 1) / b
    }

    let mut max = 0;

    for r in 0..3 {
        let cost = (cost[r] - state.resources[r]).max(0) as usize;
        let wait = ceil_divide(cost, state.robots[r] as usize);

        max = max.max(wait);

        if max == usize::MAX {
            return max;
        }
    }

    max
}

pub fn transition(blueprint: &Blueprint, state: &State, time_limit: usize) -> Vec<State> {
    let mut result = Vec::new();

    if state.time_remaining == 0 {
        return result;
    }

    if state.time_remaining > 1 {
        for r in (0..4).rev() {
            let wait = wait_time(&blueprint.robot_costs[r], state);

            let robot_mines = Resources {
                ore: (r == 0) as isize,
                clay: (r == 1) as isize,
                obsidian: (r == 2) as isize,
                geodes: (r == 3) as isize,
            };

            if wait < state.time_remaining
                && (state.robots.clone() + robot_mines.clone())[r] <= blueprint.max_resources[r]
            {
                tick(
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

    // We can't afford any robots within the time limit, so we have to wait
    // until time's up to get our final score.
    if result.is_empty() {
        tick(
            state,
            state.time_remaining,
            &Resources::default(),
            &Resources::default(),
            &mut result,
        );
    }

    result
        .iter()
        .map(|s| prune_resources(blueprint, s))
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
