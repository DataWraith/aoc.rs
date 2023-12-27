use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut state = State::new(input);

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let (s, low, high) = press_button(input, state);
        low_pulses += low;
        high_pulses += high;
        state = s;
    }

    (low_pulses * high_pulses).to_string()
}

pub fn press_button(input: &PuzzleInput, state: State) -> (State, usize, usize) {
    let mut state = state;
    let mut queue = VecDeque::new();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    queue.push_front(("button".to_string(), "roadcaster".to_string(), Pulse::Low));
    state.button_presses += 1;

    while let Some((from, cur, pulse)) = queue.pop_front() {
        if pulse == Pulse::Low {
            low_pulses += 1;
        } else {
            high_pulses += 1;
        }

        let node = input
            .graph
            .node_indices()
            .find(|&n| input.graph[n] == cur)
            .unwrap();

        match input.node_types.get(&cur) {
            Some('b') => {
                for target in input.graph.neighbors(node) {
                    queue.push_back((cur.clone(), input.graph[target].clone(), pulse));
                }
            }

            Some('%') => {
                if pulse == Pulse::High {
                    continue;
                }

                let flip_flop_state = state.s.get_mut(&cur).unwrap();

                match flip_flop_state {
                    ModuleState::FlipFlop { on } => {
                        let next_pulse = if *on { Pulse::Low } else { Pulse::High };

                        for target in input.graph.neighbors(node) {
                            queue.push_back((cur.clone(), input.graph[target].clone(), next_pulse));
                        }

                        *on = !*on;
                    }

                    _ => unreachable!(),
                }
            }

            Some('&') => {
                let module = state.s.get_mut(&cur).unwrap();

                match module {
                    ModuleState::Conjunction { memory } => {
                        memory.insert(from.clone(), pulse);

                        let next_pulse = if memory.values().any(|&p| p == Pulse::Low) {
                            if !state.conjunction_cycles.contains_key(&cur) {
                                state
                                    .conjunction_cycles
                                    .insert(cur.clone(), state.button_presses);
                            }

                            Pulse::High
                        } else {
                            Pulse::Low
                        };

                        for target in input.graph.neighbors(node) {
                            queue.push_back((cur.clone(), input.graph[target].clone(), next_pulse));
                        }
                    }

                    _ => unreachable!(),
                }
            }

            None => {}

            _ => unreachable!(),
        }
    }

    (state, low_pulses, high_pulses)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    const TEST_INPUT1: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    const TEST_INPUT2: &str = indoc! {"
       broadcaster -> a
       %a -> inv, con
       &inv -> b
       %b -> con
       &con -> output
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT1);
        assert_eq!(part1(&input), "32000000");

        let input = crate::parser::parse(TEST_INPUT2);
        assert_eq!(part1(&input), "11687500");
    }
}
