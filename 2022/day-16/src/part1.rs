use std::collections::BTreeSet;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    pub fn releasable_pressure(
        input: &PuzzleInput,
        state: State,
        max_pressure: &mut u32,
        cache: &mut HashMap<State, u32>,
    ) -> u32 {
        if let Some(result) = cache.get(&state) {
            return *result;
        }

        if state.time_left == 0 {
            let result = state.pressure_released;
            *max_pressure = (*max_pressure).max(result);
            cache.insert(state, result);
            return result;
        }

        // Prune by comparing against the best result so far.
        let mut n = state.time_left as u32;
        let mut upper_bound = state.pressure_released as u32 + state.open_valves as u32 * n;

        for (valve_id, flow_rate) in input.valve_pressures.iter() {
            if n == 0 {
                break;
            }

            if !state.opened.contains(valve_id) {
                n = n - 1;
                upper_bound += n * *flow_rate;
                n = n.saturating_sub(1);
            }
        }

        if upper_bound <= *max_pressure {
            return state.pressure_released;
        }

        let mut result = 0;

        for neighbor in input.network.neighbors(state.position) {
            let mut new_state = state.clone();

            // Go to valve
            new_state.time_left -= 1;
            new_state.pressure_released += state.open_valves;
            new_state.position = neighbor;

            // Choose not to open the valve that's here
            result = result.max(releasable_pressure(
                input,
                new_state.clone(),
                max_pressure,
                cache,
            ));

            // Open the valve that's here
            if !state.opened.contains(&neighbor) && new_state.time_left != 0 {
                // Open valve
                new_state.time_left -= 1;
                new_state.pressure_released += state.open_valves;

                // Valve is now open
                new_state.open_valves += input.network.node_weight(neighbor).unwrap_or(&0);
                new_state.opened.insert(neighbor);

                result = result.max(releasable_pressure(input, new_state, max_pressure, cache));
            }
        }

        cache.insert(state, result);
        *max_pressure = (*max_pressure).max(result);

        result
    }

    let mut max_pressure = 0;

    let mut uninteresting_valves = BTreeSet::default();

    for valve_id in input.valve_ids.values() {
        if input.network.node_weight(*valve_id).unwrap_or(&0) == &0 {
            uninteresting_valves.insert(*valve_id);
        }
    }

    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 30,
        opened: uninteresting_valves,
        pressure_released: 0,
        open_valves: 0,
    };

    releasable_pressure(
        input,
        initial_state,
        &mut max_pressure,
        &mut HashMap::default(),
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "1651");
    }
}
