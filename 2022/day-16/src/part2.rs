use crate::{part1::open_valve, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 26,
        opened: MiniBitset::<u16>::default(),
        pressure_released: 0,
        open_valves: 0,
    };

    let mut beamsearch = BeamSearch::new(2222, vec![((initial_state, initial_state), 0)]);

    let mut successors = |(myself, elephant): &(State, State)| {
        let mut result = Vec::new();

        for (i, (valve, _flow_rate)) in input.valve_pressures.iter().enumerate() {
            if myself.opened.contains(i) || elephant.opened.contains(i) {
                continue;
            }

            if myself.time_left >= elephant.time_left {
                if let Some(new_state) = open_valve(input, myself, valve, i) {
                    result.push((
                        (new_state, *elephant),
                        idle_until_deadline(&new_state, elephant),
                    ));
                }
            } else if let Some(new_state) = open_valve(input, elephant, valve, i) {
                result.push((
                    (*myself, new_state),
                    idle_until_deadline(myself, &new_state),
                ));
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

fn idle_until_deadline(myself: &State, elephant: &State) -> u16 {
    let my_pressure = myself.pressure_released + myself.open_valves * myself.time_left as u16;
    let elephant_pressure =
        elephant.pressure_released + elephant.open_valves * elephant.time_left as u16;

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
