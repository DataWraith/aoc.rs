use std::collections::BTreeSet;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    pub fn releasable_pressure(
        input: &PuzzleInput,
        my_state: State,
        elephant_state: State,
        max_pressure: &mut u32,
        cache: &mut HashMap<(State, State), u32>,
    ) -> u32 {
        if let Some(result) = cache.get(&(my_state.clone(), elephant_state.clone())) {
            return *result;
        }

        if my_state.time_left == 0 && elephant_state.time_left == 0 {
            let result = my_state.pressure_released + elephant_state.pressure_released;
            if result > *max_pressure {
                *max_pressure = result;
                dbg!((&my_state.time_left, &elephant_state.time_left, result));
            }
            *max_pressure = (*max_pressure).max(result);
            cache.insert((my_state, elephant_state), result);
            return result;
        }

        // Prune by comparing against the best result so far.
        let mut upper_bound =
            my_state.pressure_released as u32 + my_state.open_valves as u32 * my_state.time_left;
        upper_bound += elephant_state.pressure_released as u32
            + elephant_state.open_valves as u32 * elephant_state.time_left;

        let mut n = my_state.time_left.max(elephant_state.time_left);

        let mut opened = 0;
        for (valve_id, flow_rate) in input.valve_pressures.iter() {
            if n == 0 {
                break;
            }

            if !my_state.opened.contains(valve_id) {
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
        if my_state.time_left >= elephant_state.time_left {
            for (valve, _) in input.valve_pressures.iter() {
                if my_state.opened.contains(&valve) {
                    continue;
                }

                let mut new_state = my_state.clone();

                let distance = *input.distances.get(&(new_state.position, *valve)).unwrap();

                if distance >= new_state.time_left + 1 {
                    continue;
                }

                // Go to valve
                new_state.time_left -= distance;
                new_state.pressure_released += new_state.open_valves * distance;
                new_state.position = *valve;

                // Open valve
                new_state.time_left -= 1;
                new_state.pressure_released += new_state.open_valves;

                // Valve is now open
                new_state.open_valves += input.network.node_weight(*valve).unwrap();
                new_state.opened.insert(*valve);

                result = result.max(releasable_pressure(
                    input,
                    new_state,
                    elephant_state.clone(),
                    max_pressure,
                    cache,
                ));
            }
        }

        // Elephant's move
        if my_state.time_left < elephant_state.time_left {
            for (valve, _) in input.valve_pressures.iter() {
                if my_state.opened.contains(valve) {
                    continue;
                }

                let mut new_state = elephant_state.clone();

                let distance = *input.distances.get(&(new_state.position, *valve)).unwrap();

                if distance >= new_state.time_left + 1 {
                    continue;
                }

                // Go to valve
                new_state.time_left -= distance;
                new_state.pressure_released += new_state.open_valves * distance;
                new_state.position = *valve;

                // Open valve
                new_state.time_left -= 1;
                new_state.pressure_released += new_state.open_valves;

                // Valve is now open
                let mut ms = my_state.clone();
                ms.opened.insert(*valve);
                new_state.open_valves += input.network.node_weight(*valve).unwrap();

                result = result.max(releasable_pressure(
                    input,
                    ms,
                    new_state,
                    max_pressure,
                    cache,
                ));
            }
        }

        let r = my_state.pressure_released + elephant_state.pressure_released;
        let r = r + my_state.open_valves * my_state.time_left;
        let r = r + elephant_state.open_valves * elephant_state.time_left;

        result = result.max(r);

        if result > *max_pressure {
            *max_pressure = result;
            dbg!((&my_state.time_left, &elephant_state.time_left, result));
        }

        cache.insert((my_state, elephant_state), result);

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

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "1707");
    }
}
