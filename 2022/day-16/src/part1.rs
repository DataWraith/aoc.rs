use crate::structs::*;

use min_max_heap::MinMaxHeap;
use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 30,
        opened: Set64::default(),
        pressure_released: 0,
        open_valves: 0,
    };

    let beam_size = 200;
    let mut max_pressure = 0;

    let mut cur_heap = MinMaxHeap::with_capacity(beam_size);
    let mut next_heap = MinMaxHeap::with_capacity(beam_size);

    cur_heap.push((0, CmpEq(initial_state)));

    loop {
        while let Some((score, CmpEq(state))) = cur_heap.pop_max() {
            if state.time_left == 0 {
                max_pressure = max_pressure.max(score);
                continue;
            }

            let mut found = false;

            for (valve, _flow_rate) in input.valve_pressures.iter() {
                if state.opened.contains(valve.index()) {
                    continue;
                }

                if let Some(new_state) = open_valve(input, &state, valve) {
                    found = true;

                    next_heap.push((idle_until_deadline(&new_state), CmpEq(new_state)));
                }
            }

            if !found {
                max_pressure = max_pressure.max(score);
            }
        }

        while next_heap.len() > beam_size {
            next_heap.pop_min();
        }

        std::mem::swap(&mut cur_heap, &mut next_heap);

        if cur_heap.is_empty() {
            break;
        }
    }

    max_pressure.to_string()
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
