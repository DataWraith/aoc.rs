use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    pub fn releasable_pressure(
        input: &PuzzleInput,
        myself: State,
        elephant: State,
        max_pressure: &mut u32,
        cache: &mut HashMap<(State, State), u32>,
    ) -> u32 {
        if let Some(result) = cache.get(&(myself.clone(), elephant.clone())) {
            return *result;
        }

        if myself.time_left == 0 && elephant.time_left == 0 {
            let result = myself.pressure_released + elephant.pressure_released;
            *max_pressure = (*max_pressure).max(result);
            cache.insert((myself, elephant), result);
            return result;
        }

        // Prune by comparing against the best result so far.
        let mut upper_bound = myself.pressure_released + myself.open_valves * myself.time_left;
        upper_bound += elephant.pressure_released + elephant.open_valves * elephant.time_left;

        let mut n = myself.time_left.max(elephant.time_left);

        let mut opened = 0;
        for (valve_id, flow_rate) in input.valve_pressures.iter() {
            if n == 0 {
                break;
            }

            if !myself.opened.contains(valve_id.index())
                && !elephant.opened.contains(valve_id.index())
            {
                upper_bound += (n - 1) * *flow_rate;
                opened += 1;

                if opened % 2 == 0 {
                    n -= 2;
                }
            }
        }

        if upper_bound <= *max_pressure {
            return 0;
        }

        let mut result = 0;

        // My move
        if myself.time_left >= elephant.time_left {
            for (valve, _) in input.valve_pressures.iter() {
                if myself.opened.contains(valve.index()) || elephant.opened.contains(valve.index())
                {
                    continue;
                }

                if let Some(new_state) = open_valve(input, &myself, valve) {
                    result = result.max(releasable_pressure(
                        input,
                        new_state,
                        elephant.clone(),
                        max_pressure,
                        cache,
                    ));
                }
            }
        }

        // Elephant's move
        if myself.time_left < elephant.time_left {
            for (valve, _) in input.valve_pressures.iter() {
                if myself.opened.contains(valve.index()) || elephant.opened.contains(valve.index())
                {
                    continue;
                }

                if let Some(new_state) = open_valve(input, &elephant, valve) {
                    result = result.max(releasable_pressure(
                        input,
                        myself.clone(),
                        new_state,
                        max_pressure,
                        cache,
                    ));
                }
            }
        }

        // This case is only really relevant when all valves are already open
        let r = myself.pressure_released + elephant.pressure_released;
        let r = r + myself.open_valves * myself.time_left;
        let r = r + elephant.open_valves * elephant.time_left;

        result = result.max(r);
        *max_pressure = (*max_pressure).max(result);

        cache.insert((myself, elephant), result);

        result
    }

    let mut max_pressure = 0;

    let mut uninteresting_valves = Set64::default();

    for valve_id in input.valve_ids.values() {
        if input.network.node_weight(*valve_id).unwrap_or(&0) == &0 {
            uninteresting_valves.insert(valve_id.index());
        }
    }

    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 26,
        opened: uninteresting_valves,
        pressure_released: 0,
        open_valves: 0,
    };

    releasable_pressure(
        input,
        initial_state.clone(),
        initial_state,
        &mut max_pressure,
        &mut HashMap::default(),
    )
    .to_string()
}

fn open_valve(input: &PuzzleInput, state: &State, valve: &petgraph::NodeIndex) -> Option<State> {
    let distance = *input.distances.get(&(state.position, *valve)).unwrap();

    // Not enough time to get there and open the valve *and* let steam escape.
    if distance + 1 >= state.time_left {
        return None;
    }

    let mut new_state = state.clone();

    // Go to valve
    new_state.time_left -= distance;
    new_state.pressure_released += new_state.open_valves * distance;
    new_state.position = *valve;

    // Open valve
    new_state.time_left -= 1;
    new_state.pressure_released += new_state.open_valves;

    // Valve is now open
    new_state.open_valves += input.network.node_weight(*valve).unwrap();
    new_state.opened.insert(valve.index());

    Some(new_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "1707");
    }
}
