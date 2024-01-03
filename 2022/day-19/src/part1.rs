use crate::structs::*;
use std::hash::{Hash, Hasher};

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .blueprints
        .iter()
        .map(|bp| solve(bp, 24))
        .max()
        .unwrap()
        .to_string()
}

pub fn bnb<N, FN, FC, FB, IN, C>(start: &N, mut successors: FN, mut cost: FC, mut bound: FB) -> N
where
    N: Eq + Clone + Hash,
    FN: FnMut(&N) -> IN,
    FC: FnMut(&N) -> Option<C>,
    FB: FnMut(&N) -> C,
    IN: IntoIterator<Item = N>,
    C: Ord + Copy,
{
    let mut stack = vec![start.clone()];
    let mut best = None;
    let mut best_n = start.clone();

    let mut seen = HashSet::default();

    while let Some(cur) = stack.pop() {
        if seen.contains(&cur) {
            continue;
        }

        seen.insert(cur.clone());

        if let Some(cost) = cost(&cur) {
            if best.is_none() || cost < best.unwrap() {
                best = Some(cost);
                best_n = cur.clone();
            }

            continue;
        }

        for next in successors(&cur) {
            if (best.is_none() || bound(&next) < best.unwrap()) {
                stack.push(next);
            }
        }
    }

    best_n
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
        current_action: Action::Wait,
    };

    let result = bnb(
        &initial_state,
        |s: &State| transition(blueprint, s, time_limit),
        |s: &State| {
            if s.time >= time_limit {
                Some(dbg!(std::cmp::Reverse(s.resources.geodes)))
            } else {
                None
            }
        },
        |s: &State| {
            let time_left = time_limit - s.time;
            let cur_geodes = s.resources.geodes;
            let cur_production = s.robots.geodes * time_left as isize;
            let potential_production = ((time_left * (time_left.saturating_sub(1))) / 2) as isize;

            std::cmp::Reverse(cur_geodes + cur_production + potential_production)
        },
    );

    result.resources.geodes
}

pub fn expand_actions(s: &State, new_robots: &Resources, result: &mut Vec<State>) {
    for action in [
        Action::OreRobot,
        Action::ClayRobot,
        Action::ObsidianRobot,
        Action::GeodeRobot,
        Action::Wait,
    ] {
        result.push(State {
            time: s.time + 1,
            resources: s.resources.clone() + s.robots.clone(),
            robots: s.robots.clone() + new_robots.clone(),
            current_action: action,
        });
    }
}

pub fn transition(blueprint: &Blueprint, state: &State, time_limit: usize) -> Vec<State> {
    let mut result = Vec::new();

    if state.time >= time_limit {
        return result;
    }

    match state.current_action {
        Action::Wait => expand_actions(
            state,
            &Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            },
            &mut result,
        ),

        Action::OreRobot => {
            if state.resources.can_afford(&blueprint.ore_robot_cost) {
                let mut new_state = state.clone();
                new_state.resources = new_state.resources - blueprint.ore_robot_cost.clone();

                expand_actions(
                    &new_state,
                    &Resources {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geodes: 0,
                    },
                    &mut result,
                );
            } else {
                result.push(State {
                    time: state.time + 1,
                    ..(state.clone())
                })
            }
        }

        Action::ClayRobot => {
            if state.resources.can_afford(&blueprint.clay_robot_cost) {
                let mut new_state = state.clone();
                new_state.resources = new_state.resources - blueprint.clay_robot_cost.clone();
                expand_actions(
                    &new_state,
                    &Resources {
                        ore: 0,
                        clay: 1,
                        obsidian: 0,
                        geodes: 0,
                    },
                    &mut result,
                );
            } else {
                result.push(State {
                    time: state.time + 1,
                    ..(state.clone())
                })
            }
        }

        Action::ObsidianRobot => {
            if state.resources.can_afford(&blueprint.obsidian_robot_cost) {
                let mut new_state = state.clone();
                new_state.resources = new_state.resources - blueprint.obsidian_robot_cost.clone();
                expand_actions(
                    &new_state,
                    &Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 1,
                        geodes: 0,
                    },
                    &mut result,
                );
            } else {
                result.push(State {
                    time: state.time + 1,
                    ..(state.clone())
                })
            }
        }

        Action::GeodeRobot => {
            if state.resources.can_afford(&blueprint.geode_robot_cost) {
                let mut new_state = state.clone();
                new_state.resources = new_state.resources - blueprint.geode_robot_cost.clone();
                expand_actions(
                    &new_state,
                    &Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geodes: 1,
                    },
                    &mut result,
                );
            } else {
                result.push(State {
                    time: state.time + 1,
                    ..(state.clone())
                })
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "12");
    }
}
