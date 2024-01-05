use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut max_pressure = 0;

    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 30,
        opened: Set64::default(),
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

pub fn releasable_pressure(
    input: &PuzzleInput,
    state: State,
    max_pressure: &mut u32,
    cache: &mut HashMap<State, u32>,
) -> u32 {
    if let Some(result) = cache.get(&state) {
        return *result;
    }

    let mut result = idle_until_deadline(&state);
    *max_pressure = (*max_pressure).max(result);

    // Base case: We're out of time or we've opened all the valves.
    if state.time_left == 0
        || state.opened.value().count_ones() == input.valve_pressures.len() as u32
    {
        cache.insert(state, result);
        return result;
    }

    // Compute upper bound on the pressure we can release from here.
    let mut n = state.time_left;
    let mut upper_bound = result;

    for (valve_id, flow_rate) in input.valve_pressures.iter() {
        if n == 0 {
            break;
        }

        if !state.opened.contains(valve_id.index()) {
            n -= 1;
            upper_bound += n * *flow_rate;
            n = n.saturating_sub(1);
        }
    }

    // Prune by comparing against the best result so far.
    if upper_bound <= *max_pressure {
        return 0;
    }

    for (valve, _flow_rate) in input.valve_pressures.iter() {
        if state.opened.contains(valve.index()) {
            continue;
        }

        if let Some(new_state) = open_valve(input, &state, valve) {
            result = result.max(releasable_pressure(input, new_state, max_pressure, cache));
            *max_pressure = (*max_pressure).max(result);
        }
    }

    cache.insert(state, result);
    *max_pressure = (*max_pressure).max(result);

    result
}

pub fn idle_until_deadline(myself: &State) -> u32 {
    myself.pressure_released + myself.open_valves * myself.time_left
}

pub fn open_valve(
    input: &PuzzleInput,
    state: &State,
    valve: &petgraph::NodeIndex,
) -> Option<State> {
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "1651");
    }
}
