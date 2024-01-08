use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 30,
        opened: MiniBitset::<u16>::default(),
        pressure_released: 0,
        open_valves: 0,
    };

    let beam_size = 200;
    let mut max_pressure = 0;

    let mut cur = Vec::with_capacity(beam_size);
    let mut next = Vec::with_capacity(beam_size);

    cur.push((0, CmpEq(initial_state)));

    loop {
        while let Some((score, CmpEq(state))) = cur.pop() {
            max_pressure = max_pressure.max(score);

            if state.time_left == 0 {
                continue;
            }

            for (i, (valve, _flow_rate)) in input.valve_pressures.iter().enumerate() {
                if state.opened.contains(i) {
                    continue;
                }

                if let Some(new_state) = open_valve(input, &state, valve, i) {
                    next.push((idle_until_deadline(&new_state), CmpEq(new_state)));
                }
            }
        }

        if next.len() > beam_size {
            next.select_nth_unstable_by_key(beam_size, |(score, _)| std::cmp::Reverse(*score));
            next.truncate(beam_size);
        }

        std::mem::swap(&mut cur, &mut next);

        if cur.is_empty() {
            break;
        }
    }

    max_pressure.to_string()
}

pub fn idle_until_deadline(myself: &State) -> u16 {
    myself.pressure_released + myself.open_valves * myself.time_left as u16
}

pub fn open_valve(
    input: &PuzzleInput,
    state: &State,
    valve: &petgraph::NodeIndex<u8>,
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
