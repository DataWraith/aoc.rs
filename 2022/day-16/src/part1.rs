use crate::structs::*;

use petgraph::graph::NodeIndex;
use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 30,
        opened: MiniBitset::<u16>::default(),
        pressure_released: 0,
        open_valves: 0,
    };

    let mut beamsearch = BeamSearch::new(200, vec![(initial_state, 0)]);

    let mut successors = |state: &State| {
        let mut result = Vec::new();

        if state.time_left == 0 {
            return result;
        }

        for (i, (valve, _flow_rate)) in input.valve_pressures.iter().enumerate() {
            if state.opened.contains(i) {
                continue;
            }

            if let Some(new_state) = open_valve(input, state, valve, i) {
                result.push((new_state, idle_until_deadline(&new_state)));
            }
        }

        result
    };

    let mut max_pressure = 0;

    while let Some((_state, pressure)) = beamsearch.next(&mut successors) {
        max_pressure = max_pressure.max(pressure);
    }

    max_pressure.to_string()
}

pub fn idle_until_deadline(myself: &State) -> u16 {
    myself.pressure_released + myself.open_valves * myself.time_left as u16
}

pub fn open_valve(
    input: &PuzzleInput,
    state: &State,
    valve: &NodeIndex<u8>,
    valve_idx: usize,
) -> Option<State> {
    let distance = *input.distances.get(&(state.position, *valve)).unwrap();

    // Not enough time to get there and open the valve *and* let steam escape.
    if distance + 1 >= state.time_left {
        return None;
    }

    let mut new_state = *state;

    // Go to valve
    new_state.time_left -= distance;
    new_state.pressure_released += new_state.open_valves * distance as u16;
    new_state.position = *valve;

    // Open valve
    new_state.time_left -= 1;
    new_state.pressure_released += new_state.open_valves;

    // Valve is now open
    new_state.open_valves += input.network.node_weight(*valve).unwrap();
    new_state.opened.insert(valve_idx);

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
