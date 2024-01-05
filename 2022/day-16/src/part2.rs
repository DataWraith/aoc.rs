use crate::{part1::open_valve, structs::*};

use utility_belt::prelude::*;

// NOTE: This is kind of verbose. There's a more elegant way to solve this by
// re-using the part 1 solution -- partition all valves into two sets, and then
// run the part 1 solution on each set. However, this is not as efficient as the
// solution below, which runs about twice as fast as that approach.
pub fn part2(input: &PuzzleInput) -> String {
    fn releasable_pressure(
        input: &PuzzleInput,
        myself: State,
        elephant: State,
        max_pressure: &mut u32,
        cache: &mut HashMap<(State, State), u32>,
    ) -> u32 {
        if let Some(result) = cache.get(&(myself.clone(), elephant.clone())) {
            return *result;
        }

        if let Some(result) = cache.get(&(elephant.clone(), myself.clone())) {
            return *result;
        }

        let mut result = idle_until_deadline(&myself, &elephant);

        // Update the best result so far
        *max_pressure = (*max_pressure).max(result);

        let out_of_time = myself.time_left == 0 && elephant.time_left == 0;
        let all_valves_open = myself.opened.value().count_ones()
            + elephant.opened.value().count_ones()
            == input.valve_pressures.len() as u32;

        // Base case: We are out of time, or all valves are open.
        if out_of_time || all_valves_open {
            cache.insert((myself, elephant), result);
            return result;
        }

        // Calculate upper bound of how much pressure we could possibly release.
        //
        // We start with our current estimate (base case where we do nothing).
        let mut upper_bound = result;
        let mut n = myself.time_left.max(elephant.time_left).saturating_sub(1);

        // Then we forward-simulate the best case where all valves we need
        // to open are adjacent to each other.
        let mut opened = 0;

        for (valve_id, flow_rate) in input.valve_pressures.iter() {
            if n == 0 {
                break;
            }

            if !myself.opened.contains(valve_id.index())
                && !elephant.opened.contains(valve_id.index())
            {
                // Open the valve to release pressure (n-1) times.
                upper_bound += (n - 1) * *flow_rate;
                opened += 1;

                // Once both myself and the elephant have opened a valve, we
                // need to move to the next valve. We subtract 2 here because we
                // need to account for the time it took to open the current
                // valve, as well as the travel time to the next valve.
                if opened % 2 == 0 {
                    n -= 2;
                }
            }
        }

        // Prune by comparing against the best result so far.
        if upper_bound <= *max_pressure {
            return 0;
        }

        // My move
        if myself.time_left >= elephant.time_left {
            for (valve, _flow_rate) in input.valve_pressures.iter() {
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
            for (valve, _flow_rate) in input.valve_pressures.iter() {
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

        *max_pressure = (*max_pressure).max(result);

        cache.insert((myself, elephant), result);

        result
    }

    let mut max_pressure = 0;

    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 26,
        opened: Set64::default(),
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
