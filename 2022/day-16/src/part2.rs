use crate::{part1::open_valve, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let initial_state = State {
        position: input.valve_ids["AA"],
        time_left: 26,
        opened: Set32::default(),
        pressure_released: 0,
        open_valves: 0,
    };

    let beam_size = 2222;
    let mut max_pressure = 0;

    let mut cur = Vec::with_capacity(beam_size);
    let mut next = Vec::with_capacity(beam_size);

    cur.push((0, CmpEq((initial_state.clone(), initial_state))));

    loop {
        while let Some((score, CmpEq((myself, elephant)))) = cur.pop() {
            max_pressure = max_pressure.max(score);

            for (i, (valve, _flow_rate)) in input.valve_pressures.iter().enumerate() {
                if myself.opened.contains(i) || elephant.opened.contains(i) {
                    continue;
                }

                if myself.time_left >= elephant.time_left {
                    if let Some(new_state) = open_valve(input, &myself, valve, i) {
                        next.push((
                            idle_until_deadline(&new_state, &elephant),
                            CmpEq((new_state, elephant.clone())),
                        ));
                    }
                } else if let Some(new_state) = open_valve(input, &elephant, valve, i) {
                    next.push((
                        idle_until_deadline(&myself, &new_state),
                        CmpEq((myself.clone(), new_state)),
                    ));
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
