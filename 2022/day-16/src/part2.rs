use crate::{part1::open_valve, structs::*};

use min_max_heap::MinMaxHeap;
use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 26,
        opened: Set64::default(),
        pressure_released: 0,
        open_valves: 0,
    };

    let beam_size = 2222;
    let mut max_pressure = 0;

    let mut cur_heap = MinMaxHeap::with_capacity(beam_size);
    let mut next_heap = MinMaxHeap::with_capacity(beam_size);

    cur_heap.push((0, CmpEq((initial_state.clone(), initial_state))));

    loop {
        while let Some((score, CmpEq((myself, elephant)))) = cur_heap.pop_max() {
            max_pressure = max_pressure.max(score);

            for (valve, _flow_rate) in input.valve_pressures.iter() {
                if myself.opened.contains(valve.index()) || elephant.opened.contains(valve.index())
                {
                    continue;
                }

                if myself.time_left >= elephant.time_left {
                    if let Some(new_state) = open_valve(input, &myself, valve) {
                        next_heap.push((
                            idle_until_deadline(&new_state, &elephant),
                            CmpEq((new_state, elephant.clone())),
                        ));
                    }
                } else if let Some(new_state) = open_valve(input, &elephant, valve) {
                    next_heap.push((
                        idle_until_deadline(&myself, &new_state),
                        CmpEq((myself.clone(), new_state)),
                    ));
                }
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

fn idle_until_deadline(myself: &State, elephant: &State) -> u32 {
    let my_pressure = myself.pressure_released + myself.open_valves * myself.time_left;
    let elephant_pressure = elephant.pressure_released + elephant.open_valves * elephant.time_left;

    my_pressure + elephant_pressure
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
